// Generated macro for console_stdout_set_echo (function)
macro_rules! Depcrate_processconsole_stdout_set_echo {
() => {
// Module: crate::process
// Provides: {"console_stdout_set_echo"}
// Dependencies: {}
fn console_stdout_set_echo (on : bool) -> Result < () , Error > { let stdout_h = stdout_handle () ? ; let mut mode = CONSOLE_MODE :: default () ; unsafe { GetConsoleMode (stdout_h , & mut mode) ? } ; match on { true => mode |= ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT , false => mode &= ! ENABLE_ECHO_INPUT , } ; unsafe { SetConsoleMode (stdout_h , mode) ? ; CloseHandle (stdout_h) ? ; } Ok (()) }
};
}
