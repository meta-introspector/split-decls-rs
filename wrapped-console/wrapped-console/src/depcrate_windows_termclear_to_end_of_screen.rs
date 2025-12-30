// Generated macro for clear_to_end_of_screen (function)
macro_rules! Depcrate_windows_termclear_to_end_of_screen {
() => {
// Module: crate::windows_term
// Provides: {"clear_to_end_of_screen"}
// Dependencies: {}
pub (crate) fn clear_to_end_of_screen (out : & Term) -> io :: Result < () > { if out . is_msys_tty { return common_term :: clear_to_end_of_screen (out) ; } if let Some ((hand , csbi)) = get_console_screen_buffer_info (as_handle (out)) { unsafe { let bottom = csbi . srWindow . Right as u32 * csbi . srWindow . Bottom as u32 ; let cells = bottom - (csbi . dwCursorPosition . X as u32 * csbi . dwCursorPosition . Y as u32) ; let pos = COORD { X : 0 , Y : csbi . dwCursorPosition . Y , } ; let mut written = 0 ; FillConsoleOutputCharacterA (hand , b' ' as i8 , cells , pos , & mut written) ; FillConsoleOutputAttribute (hand , csbi . wAttributes , cells , pos , & mut written) ; SetConsoleCursorPosition (hand , pos) ; } } Ok (()) }
};
}
