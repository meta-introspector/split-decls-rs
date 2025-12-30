// Generated macro for InputWrapper (struct)
macro_rules! Depcrate_inflate_output_bufferInputWrapper {
() => {
// Module: crate::inflate::output_buffer
// Provides: {"InputWrapper"}
// Dependencies: {}
# [doc = " A wrapper for the output slice used when decompressing."] # [doc = ""] # [doc = " Using this rather than `Cursor` lets us implement the writing methods directly on"] # [doc = " the buffer and lets us use a usize rather than u64 for the position which helps with"] # [doc = " performance on 32-bit systems."] # [derive (Copy , Clone)] pub struct InputWrapper < 'a > { slice : & 'a [u8] , }
};
}
