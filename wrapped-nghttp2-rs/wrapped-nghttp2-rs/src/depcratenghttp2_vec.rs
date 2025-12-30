// Generated macro for nghttp2_vec (struct)
macro_rules! Depcratenghttp2_vec {
() => {
// Module: crate
// Provides: {"nghttp2_vec"}
// Dependencies: {}
# [doc = " @struct"] # [doc = ""] # [doc = " The object representing single contiguous buffer."] # [repr (C)] # [derive (Debug , Copy , Clone)] pub struct nghttp2_vec { # [doc = " The pointer to the buffer."] pub base : * mut u8 , # [doc = " The length of the buffer."] pub len : usize , }
};
}
