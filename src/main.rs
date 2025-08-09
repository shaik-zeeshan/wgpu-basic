#[cfg(target_arch = "wasm32")]
use web_sys::*;

fn main() {
    // Soon
    winit_video_player::run().expect("could not run the app");
}
