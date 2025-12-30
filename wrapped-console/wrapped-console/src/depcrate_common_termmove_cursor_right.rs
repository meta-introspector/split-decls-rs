// Generated macro for move_cursor_right (function)
macro_rules! Depcrate_common_termmove_cursor_right {
() => {
// Module: crate::common_term
// Provides: {"move_cursor_right"}
// Dependencies: {}
pub (crate) fn move_cursor_right (out : & Term , n : usize) -> io :: Result < () > { if n > 0 { out . write_str (& format ! ("\x1b[{n}C")) } else { Ok (()) } }
};
}
