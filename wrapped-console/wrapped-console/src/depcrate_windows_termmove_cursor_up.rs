// Generated macro for move_cursor_up (function)
macro_rules! Depcrate_windows_termmove_cursor_up {
() => {
// Module: crate::windows_term
// Provides: {"move_cursor_up"}
// Dependencies: {}
pub (crate) fn move_cursor_up (out : & Term , n : usize) -> io :: Result < () > { if out . is_msys_tty { return common_term :: move_cursor_up (out , n) ; } if let Some ((_ , csbi)) = get_console_screen_buffer_info (as_handle (out)) { move_cursor_to (out , 0 , csbi . dwCursorPosition . Y as usize - n) ? ; } Ok (()) }
};
}
