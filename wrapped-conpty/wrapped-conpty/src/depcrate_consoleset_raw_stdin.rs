// Generated macro for set_raw_stdin (function)
macro_rules! Depcrate_consoleset_raw_stdin {
() => {
// Module: crate::console
// Provides: {"set_raw_stdin"}
// Dependencies: {}
fn set_raw_stdin (stdin : HANDLE , mut mode : CONSOLE_MODE) -> WinResult < () > { mode &= ! ENABLE_ECHO_INPUT ; mode &= ! ENABLE_LINE_INPUT ; mode &= ! ENABLE_MOUSE_INPUT ; mode &= ! ENABLE_LINE_INPUT ; mode &= ! ENABLE_PROCESSED_INPUT ; mode |= ENABLE_EXTENDED_FLAGS ; mode |= ENABLE_INSERT_MODE ; mode |= ENABLE_QUICK_EDIT_MODE ; let vt_input_supported = true ; if vt_input_supported { mode |= ENABLE_VIRTUAL_TERMINAL_INPUT ; } unsafe { SetConsoleMode (stdin , mode) ? ; } Ok (()) }
};
}
