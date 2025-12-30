// Generated macro for move_cursor_to (function)
macro_rules! Depcrate_common_termmove_cursor_to {
() => {
// Module: crate::common_term
// Provides: {"move_cursor_to"}
// Dependencies: {}
# [inline] pub (crate) fn move_cursor_to (out : & Term , x : usize , y : usize) -> io :: Result < () > { out . write_str (& format ! ("\x1B[{};{}H" , y + 1 , x + 1)) }
};
}
