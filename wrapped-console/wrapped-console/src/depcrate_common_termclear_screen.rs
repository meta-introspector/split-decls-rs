// Generated macro for clear_screen (function)
macro_rules! Depcrate_common_termclear_screen {
() => {
// Module: crate::common_term
// Provides: {"clear_screen"}
// Dependencies: {}
# [inline] pub (crate) fn clear_screen (out : & Term) -> io :: Result < () > { out . write_str ("\r\x1b[2J\r\x1b[H") }
};
}
