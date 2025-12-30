// Generated macro for handle_result (function)
macro_rules! Depcratehandle_result {
() => {
// Module: crate
// Provides: {"handle_result"}
// Dependencies: {}
# [doc = " Get the result of a call to WinAPI that returns a handle or `INVALID_HANDLE_VALUE`."] # [inline] pub fn handle_result (return_value : HANDLE) -> io :: Result < HANDLE > { if return_value != INVALID_HANDLE_VALUE { Ok (return_value) } else { Err (io :: Error :: last_os_error ()) } }
};
}
