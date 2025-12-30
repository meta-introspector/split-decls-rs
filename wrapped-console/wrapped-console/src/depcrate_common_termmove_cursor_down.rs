// Generated macro for move_cursor_down (function)
macro_rules! Depcrate_common_termmove_cursor_down {
() => {
// Module: crate::common_term
// Provides: {"move_cursor_down"}
// Dependencies: {}
pub (crate) fn move_cursor_down (out : & Term , n : usize) -> io :: Result < () > { if n > 0 { out . write_str (& format ! ("\x1b[{n}B")) } else { Ok (()) } }
};
}
