// Generated macro for read_into_buffer (function)
macro_rules! Depcrate_utilread_into_buffer {
() => {
// Module: crate::util
// Provides: {"read_into_buffer"}
// Dependencies: {}
pub fn read_into_buffer (fd : RawFd , buffer : & mut [u8]) -> isize { unsafe { ffi :: read (fd , buffer . as_mut_ptr () as * mut c_void , buffer . len () as size_t ,) } }
};
}
