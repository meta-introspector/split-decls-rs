// Generated macro for clear_line (function)
macro_rules! Depcrate_windows_termclear_line {
() => {
// Module: crate::windows_term
// Provides: {"clear_line"}
// Dependencies: {}
pub (crate) fn clear_line (out : & Term) -> io :: Result < () > { if out . is_msys_tty { return common_term :: clear_line (out) ; } if let Some ((hand , csbi)) = get_console_screen_buffer_info (as_handle (out)) { unsafe { let width = csbi . srWindow . Right - csbi . srWindow . Left ; let pos = COORD { X : 0 , Y : csbi . dwCursorPosition . Y , } ; let mut written = 0 ; FillConsoleOutputCharacterA (hand , b' ' as i8 , width as u32 , pos , & mut written) ; FillConsoleOutputAttribute (hand , csbi . wAttributes , width as u32 , pos , & mut written) ; SetConsoleCursorPosition (hand , pos) ; } } Ok (()) }
};
}
