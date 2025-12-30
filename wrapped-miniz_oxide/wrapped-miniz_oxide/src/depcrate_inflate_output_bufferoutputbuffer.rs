// Generated macro for OutputBuffer (struct)
macro_rules! Depcrate_inflate_output_bufferOutputBuffer {
() => {
// Module: crate::inflate::output_buffer
// Provides: {"OutputBuffer"}
// Dependencies: {}
# [doc = " A wrapper for the output slice used when decompressing."] # [doc = ""] # [doc = " Using this rather than `Cursor` lets us implement the writing methods directly on"] # [doc = " the buffer and lets us use a usize rather than u64 for the position which helps with"] # [doc = " performance on 32-bit systems."] pub struct OutputBuffer < 'a > { slice : & 'a mut [u8] , position : usize , max : usize , }
};
}
