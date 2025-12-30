// Generated macro for rgb_value (function)
macro_rules! Depcratergb_value {
() => {
// Module: crate
// Provides: {"rgb_value"}
// Dependencies: {}
fn rgb_value (color : anstyle :: Color , palette : Palette) -> String { let color = anstyle_lossy :: color_to_rgb (color , palette) ; let anstyle :: RgbColor (r , g , b) = color ; format ! ("#{r:02X}{g:02X}{b:02X}") }
};
}
