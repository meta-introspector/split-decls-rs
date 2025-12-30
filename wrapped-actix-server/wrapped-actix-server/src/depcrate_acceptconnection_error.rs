// Generated macro for connection_error (function)
macro_rules! Depcrate_acceptconnection_error {
() => {
// Module: crate::accept
// Provides: {"connection_error"}
// Dependencies: {}
# [doc = " This function defines errors that are per-connection; if we get this error from the `accept()`"] # [doc = " system call it means the next connection might be ready to be accepted."] # [doc = ""] # [doc = " All other errors will incur a timeout before next `accept()` call is attempted. The timeout is"] # [doc = " useful to handle resource exhaustion errors like `ENFILE` and `EMFILE`. Otherwise, it could"] # [doc = " enter into a temporary spin loop."] fn connection_error (err : & io :: Error) -> bool { err . kind () == io :: ErrorKind :: ConnectionRefused || err . kind () == io :: ErrorKind :: ConnectionAborted || err . kind () == io :: ErrorKind :: ConnectionReset }
};
}
