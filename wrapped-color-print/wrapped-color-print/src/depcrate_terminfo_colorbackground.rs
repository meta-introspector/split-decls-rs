// Generated macro for background (function)
macro_rules! Depcrate_terminfo_colorbackground {
() => {
// Module: crate::terminfo::color
// Provides: {"background"}
// Dependencies: {}
# [doc = " Gets the ANSI code which sets the background color to the given color (0 to 15 included)."] fn background (v : u8) -> String { # [cfg (debug_assertions)] assert ! (v < 16) ; expand1_string :: < cap :: SetABackground > (v) }
};
}
