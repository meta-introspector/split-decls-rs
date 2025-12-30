// Generated macro for read_bytes (function)
macro_rules! Depcrate_unix_termread_bytes {
() => {
// Module: crate::unix_term
// Provides: {"read_bytes"}
// Dependencies: {}
fn read_bytes (fd : RawFd , buf : & mut [u8] , count : u8) -> io :: Result < u8 > { let read = unsafe { libc :: read (fd , buf . as_mut_ptr () as * mut _ , count as usize) } ; if read < 0 { Err (io :: Error :: last_os_error ()) } else if read == 0 { Err (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , "Reached end of file" ,)) } else if buf [0] == b'\x03' { Err (io :: Error :: new (io :: ErrorKind :: Interrupted , "read interrupted" ,)) } else { Ok (read as u8) } }
};
}
