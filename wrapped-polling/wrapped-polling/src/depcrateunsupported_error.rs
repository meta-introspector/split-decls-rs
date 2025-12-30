// Generated macro for unsupported_error (function)
macro_rules! Depcrateunsupported_error {
() => {
// Module: crate
// Provides: {"unsupported_error"}
// Dependencies: {}
# [allow (unused)] fn unsupported_error (err : impl Into < String >) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Unsupported , err . into ()) }
};
}
