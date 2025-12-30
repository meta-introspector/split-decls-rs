// Generated macro for color_to_xterm (function)
macro_rules! Depcratecolor_to_xterm {
() => {
// Module: crate
// Provides: {"color_to_xterm"}
// Dependencies: {}
# [doc = " Lossily convert from any color to 256-color"] # [doc = ""] # [doc = " As the palette for 4-bit colors is terminal/user defined, a [`palette::Palette`] must be"] # [doc = " provided to match against."] pub const fn color_to_xterm (color : anstyle :: Color) -> anstyle :: Ansi256Color { match color { anstyle :: Color :: Ansi (color) => anstyle :: Ansi256Color :: from_ansi (color) , anstyle :: Color :: Ansi256 (color) => color , anstyle :: Color :: Rgb (color) => rgb_to_xterm (color) , } }
};
}
