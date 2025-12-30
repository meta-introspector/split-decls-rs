// Generated macro for read_from_pipe (function)
macro_rules! Depcrate_io_readerread_from_pipe {
() => {
// Module: crate::io::reader
// Provides: {"read_from_pipe"}
// Dependencies: {}
fn read_from_pipe (h : HANDLE , buf : & mut [u8]) -> io :: Result < usize > { let mut n = 0 ; unsafe { ReadFile (h , Some (buf) , Some (& mut n) , None) ? ; } Ok (n as usize) }
};
}
