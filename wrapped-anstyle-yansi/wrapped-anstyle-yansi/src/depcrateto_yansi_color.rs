// Generated macro for to_yansi_color (function)
macro_rules! Depcrateto_yansi_color {
() => {
// Module: crate
// Provides: {"to_yansi_color"}
// Dependencies: {}
# [doc = " Adapt generic color to [`yansi`]"] pub fn to_yansi_color (color : anstyle :: Color) -> yansi :: Color { match color { anstyle :: Color :: Ansi (ansi) => ansi_to_yansi_color (ansi) , anstyle :: Color :: Ansi256 (xterm) => xterm_to_yansi_color (xterm) , anstyle :: Color :: Rgb (rgb) => rgb_to_yansi_color (rgb) , } }
};
}
