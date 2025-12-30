// Generated macro for rgb_to_ansi_color (function)
macro_rules! Depcratergb_to_ansi_color {
() => {
// Module: crate
// Provides: {"rgb_to_ansi_color"}
// Dependencies: {}
fn rgb_to_ansi_color (color : anstyle :: RgbColor) -> crossterm :: style :: Color { crossterm :: style :: Color :: Rgb { r : color . 0 , g : color . 1 , b : color . 2 , } }
};
}
