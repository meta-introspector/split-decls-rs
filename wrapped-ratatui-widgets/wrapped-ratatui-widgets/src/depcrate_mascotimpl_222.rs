// Generated macro for impl_222 (impl)
macro_rules! Depcrate_mascotimpl_222 {
() => {
// Module: crate::mascot
// Provides: {"impl_222"}
// Dependencies: {}
impl RatatuiMascot { # [doc = " Create a new Ratatui mascot widget"] pub fn new () -> Self { Self { .. Default :: default () } } # [doc = " Set the eye state (open / blinking)"] # [must_use] pub const fn set_eye (self , rat_eye : MascotEyeColor) -> Self { Self { eye_state : rat_eye , .. self } } const fn color_for (& self , c : char) -> Option < Color > { match c { RAT => Some (self . rat_color) , HAT => Some (self . hat_color) , EYE => Some (match self . eye_state { MascotEyeColor :: Default => self . rat_eye_color , MascotEyeColor :: Red => self . rat_eye_blink , }) , TERM => Some (self . term_color) , TERM_CURSOR => Some (self . term_cursor_color) , TERM_BORDER => Some (self . term_border_color) , _ => None , } } }
};
}
