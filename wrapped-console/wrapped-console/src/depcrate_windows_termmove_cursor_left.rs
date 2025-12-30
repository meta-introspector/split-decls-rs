// Generated macro for move_cursor_left (function)
macro_rules! Depcrate_windows_termmove_cursor_left {
() => {
// Module: crate::windows_term
// Provides: {"move_cursor_left"}
// Dependencies: {}
pub (crate) fn move_cursor_left (out : & Term , n : usize) -> io :: Result < () > { if out . is_msys_tty { return common_term :: move_cursor_left (out , n) ; } if let Some ((_ , csbi)) = get_console_screen_buffer_info (as_handle (out)) { move_cursor_to (out , csbi . dwCursorPosition . X as usize - n , csbi . dwCursorPosition . Y as usize ,) ? ; } Ok (()) }
};
}
