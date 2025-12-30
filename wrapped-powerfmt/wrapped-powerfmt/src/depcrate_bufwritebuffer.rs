// Generated macro for WriteBuffer (struct)
macro_rules! Depcrate_bufWriteBuffer {
() => {
// Module: crate::buf
// Provides: {"WriteBuffer"}
// Dependencies: {}
# [doc = " A buffer for construct a string while avoiding heap allocation."] # [doc = ""] # [doc = " The only requirement is that the buffer is large enough to hold the formatted string."] pub struct WriteBuffer < const SIZE : usize > { buf : [MaybeUninit < u8 > ; SIZE] , len : usize , }
};
}
