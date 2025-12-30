// Generated macro for BlockBuffer (struct)
macro_rules! Depcrate_decoderBlockBuffer {
() => {
// Module: crate::decoder
// Provides: {"BlockBuffer"}
// Dependencies: {}
# [doc = " Base64 decode buffer for a 1-block input."] # [doc = ""] # [doc = " This handles a partially decoded block of data, i.e. data which has been"] # [doc = " decoded but not read."] # [derive (Clone , Default , Debug)] struct BlockBuffer { # [doc = " 3 decoded bytes from a 4-byte Base64-encoded input."] decoded : [u8 ; Self :: SIZE] , # [doc = " Length of the buffer."] length : usize , # [doc = " Position within the buffer."] position : usize , }
};
}
