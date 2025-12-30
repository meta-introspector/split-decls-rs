// Generated macro for nonnull_handle_result (function)
macro_rules! Depcratenonnull_handle_result {
() => {
// Module: crate
// Provides: {"nonnull_handle_result"}
// Dependencies: {}
# [doc = " Get the result of a call to WinAPI that returns a handle or `NULL`."] # [inline] pub fn nonnull_handle_result (return_value : HANDLE) -> io :: Result < HANDLE > { if return_value . is_null () { Err (io :: Error :: last_os_error ()) } else { Ok (return_value) } }
};
}
