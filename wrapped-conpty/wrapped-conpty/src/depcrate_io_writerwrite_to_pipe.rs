// Generated macro for write_to_pipe (function)
macro_rules! Depcrate_io_writerwrite_to_pipe {
() => {
// Module: crate::io::writer
// Provides: {"write_to_pipe"}
// Dependencies: {}
fn write_to_pipe (h : HANDLE , buf : & [u8]) -> io :: Result < usize > { let mut n = 0 ; unsafe { WriteFile (h , Some (buf) , Some (& mut n) , None) ? ; } Ok (n as usize) }
};
}
