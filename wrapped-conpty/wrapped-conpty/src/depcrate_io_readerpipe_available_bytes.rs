// Generated macro for pipe_available_bytes (function)
macro_rules! Depcrate_io_readerpipe_available_bytes {
() => {
// Module: crate::io::reader
// Provides: {"pipe_available_bytes"}
// Dependencies: {}
fn pipe_available_bytes (h : HANDLE) -> io :: Result < u32 > { let mut bytes = MaybeUninit :: < u32 > :: uninit () ; let bytes_ptr : * mut u32 = unsafe { ptr :: addr_of_mut ! (* bytes . as_mut_ptr ()) } ; unsafe { PeekNamedPipe (h , None , 0 , None , Some (bytes_ptr) , None) ? ; } let bytes = unsafe { bytes . assume_init () } ; Ok (bytes) }
};
}
