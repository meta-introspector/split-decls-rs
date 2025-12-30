// Generated macro for initialize_buffer (function)
macro_rules! Depcrate_ffiinitialize_buffer {
() => {
// Module: crate::ffi
// Provides: {"initialize_buffer"}
// Dependencies: {}
fn initialize_buffer (output : & mut [MaybeUninit < u8 >]) -> & mut [u8] { unsafe { output . as_mut_ptr () . write_bytes (0 , output . len ()) ; & mut * (output as * mut [MaybeUninit < u8 >] as * mut [u8]) } }
};
}
