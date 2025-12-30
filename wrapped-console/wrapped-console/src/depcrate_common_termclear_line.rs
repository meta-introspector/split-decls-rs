// Generated macro for clear_line (function)
macro_rules! Depcrate_common_termclear_line {
() => {
// Module: crate::common_term
// Provides: {"clear_line"}
// Dependencies: {}
# [inline] pub (crate) fn clear_line (out : & Term) -> io :: Result < () > { out . write_str ("\r\x1b[2K") }
};
}
