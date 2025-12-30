// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl FpsWidget { # [doc = " Update the fps calculation."] # [doc = ""] # [doc = " This updates the fps once a second, but only if the widget has rendered at least 2 frames"] # [doc = " since the last calculation. This avoids noise in the fps calculation when rendering on slow"] # [doc = " machines that can't render at least 2 frames per second."] # [expect (clippy :: cast_precision_loss)] fn calculate_fps (& mut self) { self . frame_count += 1 ; let elapsed = self . last_instant . elapsed () ; if elapsed > Duration :: from_secs (1) && self . frame_count > 2 { self . fps = Some (self . frame_count as f32 / elapsed . as_secs_f32 ()) ; self . frame_count = 0 ; self . last_instant = Instant :: now () ; } } }
};
}
