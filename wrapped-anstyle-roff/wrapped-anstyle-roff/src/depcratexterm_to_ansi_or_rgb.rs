// Generated macro for xterm_to_ansi_or_rgb (function)
macro_rules! Depcratexterm_to_ansi_or_rgb {
() => {
// Module: crate
// Provides: {"xterm_to_ansi_or_rgb"}
// Dependencies: {}
# [doc = " Non Lossy Conversion of Xterm color to one that Roff can handle"] fn xterm_to_ansi_or_rgb (color : Ansi256Color) -> Color { match color . into_ansi () { Some (ansi_color) => Color :: Ansi (ansi_color) , None => Color :: Rgb (anstyle_lossy :: xterm_to_rgb (color , Palette :: default ())) , } }
};
}
