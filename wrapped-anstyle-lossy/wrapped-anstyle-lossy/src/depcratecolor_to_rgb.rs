// Generated macro for color_to_rgb (function)
macro_rules! Depcratecolor_to_rgb {
() => {
// Module: crate
// Provides: {"color_to_rgb"}
// Dependencies: {}
# [doc = " Lossily convert from any color to RGB"] # [doc = ""] # [doc = " As the palette for 4-bit colors is terminal/user defined, a [`palette::Palette`] must be"] # [doc = " provided to match against."] pub const fn color_to_rgb (color : anstyle :: Color , palette : palette :: Palette) -> anstyle :: RgbColor { match color { anstyle :: Color :: Ansi (color) => ansi_to_rgb (color , palette) , anstyle :: Color :: Ansi256 (color) => xterm_to_rgb (color , palette) , anstyle :: Color :: Rgb (color) => color , } }
};
}
