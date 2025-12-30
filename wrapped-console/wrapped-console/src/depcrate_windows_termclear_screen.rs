// Generated macro for clear_screen (function)
macro_rules! Depcrate_windows_termclear_screen {
() => {
// Module: crate::windows_term
// Provides: {"clear_screen"}
// Dependencies: {}
pub (crate) fn clear_screen (out : & Term) -> io :: Result < () > { if out . is_msys_tty { return common_term :: clear_screen (out) ; } if let Some ((hand , csbi)) = get_console_screen_buffer_info (as_handle (out)) { unsafe { let cells = csbi . dwSize . X as u32 * csbi . dwSize . Y as u32 ; let pos = COORD { X : 0 , Y : 0 } ; let mut written = 0 ; FillConsoleOutputCharacterA (hand , b' ' as i8 , cells , pos , & mut written) ; FillConsoleOutputAttribute (hand , csbi . wAttributes , cells , pos , & mut written) ; SetConsoleCursorPosition (hand , pos) ; } } Ok (()) }
};
}
