// Generated macro for move_cursor_up (function)
macro_rules! Depcrate_common_termmove_cursor_up {
() => {
// Module: crate::common_term
// Provides: {"move_cursor_up"}
// Dependencies: {}
pub (crate) fn move_cursor_up (out : & Term , n : usize) -> io :: Result < () > { if n > 0 { out . write_str (& format ! ("\x1b[{n}A")) } else { Ok (()) } }
};
}
