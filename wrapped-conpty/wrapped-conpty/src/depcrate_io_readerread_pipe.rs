// Generated macro for read_pipe (function)
macro_rules! Depcrate_io_readerread_pipe {
() => {
// Module: crate::io::reader
// Provides: {"read_pipe"}
// Dependencies: {}
fn read_pipe (h : HANDLE , buf : & mut [u8] , blocking : bool) -> io :: Result < usize > { if ! blocking { let available = pipe_available_bytes (h) ? ; if available == 0 { return Err (io :: Error :: new (io :: ErrorKind :: WouldBlock , "")) ; } } read_from_pipe (h , buf) }
};
}
