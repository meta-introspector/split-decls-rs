// Generated macro for Buffer (struct)
macro_rules! DepcrateBuffer {
() => {
// Module: crate
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " Buffer represents an owned chunk of memory on the BoringSSL heap."] # [doc = " Call `as_ref()` to get a `&[u8]` from it."] pub struct Buffer { ptr : * mut u8 , len : usize , }
};
}
