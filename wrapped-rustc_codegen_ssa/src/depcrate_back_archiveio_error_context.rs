// Generated macro for io_error_context (function)
macro_rules! Depcrate_back_archiveio_error_context {
() => {
// Module: crate::back::archive
// Provides: {"io_error_context"}
// Dependencies: {}
fn io_error_context (context : & str , err : io :: Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , format ! ("{context}: {err}")) }
};
}
