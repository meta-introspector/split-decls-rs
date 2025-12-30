// Generated macro for clear_to_end_of_screen (function)
macro_rules! Depcrate_common_termclear_to_end_of_screen {
() => {
// Module: crate::common_term
// Provides: {"clear_to_end_of_screen"}
// Dependencies: {}
# [inline] pub (crate) fn clear_to_end_of_screen (out : & Term) -> io :: Result < () > { out . write_str ("\r\x1b[0J") }
};
}
