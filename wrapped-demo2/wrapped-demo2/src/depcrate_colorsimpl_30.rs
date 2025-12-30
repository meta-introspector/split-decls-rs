// Generated macro for impl_30 (impl)
macro_rules! Depcrate_colorsimpl_30 {
() => {
// Module: crate::colors
// Provides: {"impl_30"}
// Dependencies: {}
impl Widget for RgbSwatch { # [expect (clippy :: cast_precision_loss , clippy :: similar_names)] fn render (self , area : Rect , buf : & mut Buffer) { for (yi , y) in (area . top () .. area . bottom ()) . enumerate () { let value = f32 :: from (area . height) - yi as f32 ; let value_fg = value / f32 :: from (area . height) ; let value_bg = (value - 0.5) / f32 :: from (area . height) ; for (xi , x) in (area . left () .. area . right ()) . enumerate () { let hue = xi as f32 * 360.0 / f32 :: from (area . width) ; let fg = color_from_oklab (hue , Okhsv :: max_saturation () , value_fg) ; let bg = color_from_oklab (hue , Okhsv :: max_saturation () , value_bg) ; buf [(x , y)] . set_char ('▀') . set_fg (fg) . set_bg (bg) ; } } } }
};
}
