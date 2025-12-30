// Generated macro for color_to_ansi (function)
macro_rules! Depcratecolor_to_ansi {
() => {
// Module: crate
// Provides: {"color_to_ansi"}
// Dependencies: {}
# [doc = " Lossily convert from any color to 4-bit color"] # [doc = ""] # [doc = " As the palette for 4-bit colors is terminal/user defined, a [`palette::Palette`] must be"] # [doc = " provided to match against."] pub const fn color_to_ansi (color : anstyle :: Color , palette : palette :: Palette) -> anstyle :: AnsiColor { match color { anstyle :: Color :: Ansi (color) => color , anstyle :: Color :: Ansi256 (color) => xterm_to_ansi (color , palette) , anstyle :: Color :: Rgb (color) => rgb_to_ansi (color , palette) , } }
};
}
