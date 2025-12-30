// Generated macro for terminal_size (function)
macro_rules! Depcrate_windows_termterminal_size {
() => {
// Module: crate::windows_term
// Provides: {"terminal_size"}
// Dependencies: {}
pub (crate) fn terminal_size (out : & Term) -> Option < (u16 , u16) > { use windows_sys :: Win32 :: System :: Console :: SMALL_RECT ; let handle = out . as_raw_handle () ; let hand = handle as windows_sys :: Win32 :: Foundation :: HANDLE ; if hand == INVALID_HANDLE_VALUE { return None ; } let zc = COORD { X : 0 , Y : 0 } ; let mut csbi = CONSOLE_SCREEN_BUFFER_INFO { dwSize : zc , dwCursorPosition : zc , wAttributes : 0 , srWindow : SMALL_RECT { Left : 0 , Top : 0 , Right : 0 , Bottom : 0 , } , dwMaximumWindowSize : zc , } ; if unsafe { GetConsoleScreenBufferInfo (hand , & mut csbi) } == 0 { return None ; } let rows = (csbi . srWindow . Bottom - csbi . srWindow . Top + 1) as u16 ; let columns = (csbi . srWindow . Right - csbi . srWindow . Left + 1) as u16 ; Some ((rows , columns)) }
};
}
