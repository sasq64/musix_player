use std::{
    error::Error,
    path::{Path, PathBuf},
};

use cpal::traits::*;

fn main() -> Result<(), Box<dyn Error>> {
    let song_file = std::env::args().nth(1).expect("Need song file.");
    let song_path = PathBuf::from(song_file);

    musix::init(Path::new("data")).unwrap();

    // Use cpal to set up an audio device with 2 channels at 44100 HZ
    let device = cpal::default_host().default_output_device().unwrap();
    let mut config = device.default_output_config().unwrap();
    let configs = device.supported_output_configs().unwrap();
    for conf in configs {
        if let Some(conf2) = conf.try_with_sample_rate(cpal::SampleRate(44100)) {
            config = conf2;
            break;
        }
    }

    // Load the song
    let mut player = musix::load_song(&song_path)?;

    // Temporary buffer for output data from song player
    let mut target: Vec<i16> = vec![0; 32768];

    // Create audio stream with callback that plays song
    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                player.get_samples(&mut target[0..data.len()]);
                for i in 0..data.len() {
                    data[i] = (target[i] as f32) / 32767.0;
                }
            },
            |err| eprintln!("An error occurred on stream: {err}"),
            None,
        )
        .unwrap();
    stream.play().unwrap();

    // Wait forever (press CTRL-C to quit)
    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    #[allow(unreachable_code)]
    Ok(())
}
