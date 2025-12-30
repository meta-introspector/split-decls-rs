// Generated macro for move_cursor_to (function)
macro_rules! Depcrate_windows_termmove_cursor_to {
() => {
// Module: crate::windows_term
// Provides: {"move_cursor_to"}
// Dependencies: {}
pub (crate) fn move_cursor_to (out : & Term , x : usize , y : usize) -> io :: Result < () > { if out . is_msys_tty { return common_term :: move_cursor_to (out , x , y) ; } if let Some ((hand , _)) = get_console_screen_buffer_info (as_handle (out)) { unsafe { SetConsoleCursorPosition (hand , COORD { X : x as i16 , Y : y as i16 , } ,) ; } } Ok (()) }
};
}
