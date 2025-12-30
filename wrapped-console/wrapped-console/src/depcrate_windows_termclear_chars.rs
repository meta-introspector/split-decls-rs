// Generated macro for clear_chars (function)
macro_rules! Depcrate_windows_termclear_chars {
() => {
// Module: crate::windows_term
// Provides: {"clear_chars"}
// Dependencies: {}
pub (crate) fn clear_chars (out : & Term , n : usize) -> io :: Result < () > { if out . is_msys_tty { return common_term :: clear_chars (out , n) ; } if let Some ((hand , csbi)) = get_console_screen_buffer_info (as_handle (out)) { unsafe { let width = cmp :: min (csbi . dwCursorPosition . X , n as i16) ; let pos = COORD { X : csbi . dwCursorPosition . X - width , Y : csbi . dwCursorPosition . Y , } ; let mut written = 0 ; FillConsoleOutputCharacterA (hand , b' ' as i8 , width as u32 , pos , & mut written) ; FillConsoleOutputAttribute (hand , csbi . wAttributes , width as u32 , pos , & mut written) ; SetConsoleCursorPosition (hand , pos) ; } } Ok (()) }
};
}
