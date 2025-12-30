// Generated macro for hide_cursor (function)
macro_rules! Depcrate_common_termhide_cursor {
() => {
// Module: crate::common_term
// Provides: {"hide_cursor"}
// Dependencies: {}
# [inline] pub (crate) fn hide_cursor (out : & Term) -> io :: Result < () > { out . write_str ("\x1b[?25l") }
};
}
