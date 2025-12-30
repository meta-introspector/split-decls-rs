// Generated macro for get_console_screen_buffer_info (function)
macro_rules! Depcrate_windows_termget_console_screen_buffer_info {
() => {
// Module: crate::windows_term
// Provides: {"get_console_screen_buffer_info"}
// Dependencies: {}
fn get_console_screen_buffer_info (hand : HANDLE) -> Option < (HANDLE , CONSOLE_SCREEN_BUFFER_INFO) > { let mut csbi : CONSOLE_SCREEN_BUFFER_INFO = unsafe { mem :: zeroed () } ; match unsafe { GetConsoleScreenBufferInfo (hand , & mut csbi) } { 0 => None , _ => Some ((hand , csbi)) , } }
};
}
