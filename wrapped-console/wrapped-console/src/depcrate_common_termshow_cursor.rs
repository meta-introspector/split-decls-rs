// Generated macro for show_cursor (function)
macro_rules! Depcrate_common_termshow_cursor {
() => {
// Module: crate::common_term
// Provides: {"show_cursor"}
// Dependencies: {}
# [inline] pub (crate) fn show_cursor (out : & Term) -> io :: Result < () > { out . write_str ("\x1b[?25h") }
};
}
