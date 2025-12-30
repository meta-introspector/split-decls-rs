// Generated macro for get_console_cursor_info (function)
macro_rules! Depcrate_windows_termget_console_cursor_info {
() => {
// Module: crate::windows_term
// Provides: {"get_console_cursor_info"}
// Dependencies: {}
fn get_console_cursor_info (hand : HANDLE) -> Option < (HANDLE , CONSOLE_CURSOR_INFO) > { let mut cci : CONSOLE_CURSOR_INFO = unsafe { mem :: zeroed () } ; match unsafe { GetConsoleCursorInfo (hand , & mut cci) } { 0 => None , _ => Some ((hand , cci)) , } }
};
}
