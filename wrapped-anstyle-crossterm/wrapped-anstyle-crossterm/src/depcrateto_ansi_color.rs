// Generated macro for to_ansi_color (function)
macro_rules! Depcrateto_ansi_color {
() => {
// Module: crate
// Provides: {"to_ansi_color"}
// Dependencies: {}
fn to_ansi_color (color : anstyle :: Color) -> crossterm :: style :: Color { match color { anstyle :: Color :: Ansi (ansi) => ansi_to_ansi_color (ansi) , anstyle :: Color :: Ansi256 (xterm) => xterm_to_ansi_color (xterm) , anstyle :: Color :: Rgb (rgb) => rgb_to_ansi_color (rgb) , } }
};
}
