// Generated macro for to_termcolor_color (function)
macro_rules! Depcrateto_termcolor_color {
() => {
// Module: crate
// Provides: {"to_termcolor_color"}
// Dependencies: {}
# [doc = " Adapt generic colors to [`termcolor`]"] pub fn to_termcolor_color (color : anstyle :: Color) -> termcolor :: Color { match color { anstyle :: Color :: Ansi (ansi) => ansi_to_termcolor_color (ansi) , anstyle :: Color :: Ansi256 (xterm) => xterm_to_termcolor_color (xterm) , anstyle :: Color :: Rgb (rgb) => rgb_to_termcolor_color (rgb) , } }
};
}
