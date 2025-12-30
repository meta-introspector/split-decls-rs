// Generated macro for move_cursor_left (function)
macro_rules! Depcrate_common_termmove_cursor_left {
() => {
// Module: crate::common_term
// Provides: {"move_cursor_left"}
// Dependencies: {}
pub (crate) fn move_cursor_left (out : & Term , n : usize) -> io :: Result < () > { if n > 0 { out . write_str (& format ! ("\x1b[{n}D")) } else { Ok (()) } }
};
}
