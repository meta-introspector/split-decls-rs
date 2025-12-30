// Generated macro for no_such_file_err_msg (function)
macro_rules! Depcrateno_such_file_err_msg {
() => {
// Module: crate
// Provides: {"no_such_file_err_msg"}
// Dependencies: {}
# [doc = " The error message for ENOENT."] pub fn no_such_file_err_msg () -> String { std :: io :: Error :: from_raw_os_error (2) . to_string () }
};
}
