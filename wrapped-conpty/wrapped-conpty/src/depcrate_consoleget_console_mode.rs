// Generated macro for get_console_mode (function)
macro_rules! Depcrate_consoleget_console_mode {
() => {
// Module: crate::console
// Provides: {"get_console_mode"}
// Dependencies: {}
fn get_console_mode (h : HANDLE) -> WinResult < CONSOLE_MODE > { let mut mode = CONSOLE_MODE :: default () ; unsafe { GetConsoleMode (h , & mut mode) ? ; } Ok (mode) }
};
}
