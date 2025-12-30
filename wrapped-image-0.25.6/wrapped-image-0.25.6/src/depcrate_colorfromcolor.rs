// Generated macro for FromColor (trait)
macro_rules! Depcrate_colorFromColor {
() => {
// Module: crate::color
// Provides: {"FromColor"}
// Dependencies: {}
# [doc = " Provides color conversions for the different pixel types."] pub trait FromColor < Other > { # [doc = " Changes `self` to represent `Other` in the color space of `Self`"] # [allow (clippy :: wrong_self_convention)] fn from_color (& mut self , _ : & Other) ; }
};
}
