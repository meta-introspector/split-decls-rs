// Generated macro for get_stdin_handle (function)
macro_rules! Depcrate_windows_termget_stdin_handle {
() => {
// Module: crate::windows_term
// Provides: {"get_stdin_handle"}
// Dependencies: {}
fn get_stdin_handle () -> io :: Result < HANDLE > { let handle = unsafe { GetStdHandle (STD_INPUT_HANDLE) } ; if handle == INVALID_HANDLE_VALUE { Err (io :: Error :: last_os_error ()) } else { Ok (handle) } }
};
}
