// Generated macro for Limit (struct)
macro_rules! Depcrate_buf_limitLimit {
() => {
// Module: crate::buf::limit
// Provides: {"Limit"}
// Dependencies: {}
# [doc = " A `BufMut` adapter which limits the amount of bytes that can be written"] # [doc = " to an underlying buffer."] # [derive (Debug)] pub struct Limit < T > { inner : T , limit : usize , }
};
}
