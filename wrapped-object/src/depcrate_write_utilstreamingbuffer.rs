// Generated macro for StreamingBuffer (struct)
macro_rules! Depcrate_write_utilStreamingBuffer {
() => {
// Module: crate::write::util
// Provides: {"StreamingBuffer"}
// Dependencies: {}
# [doc = " A [`WritableBuffer`] that streams data to a [`Write`](std::io::Write) implementation."] # [doc = ""] # [doc = " [`Self::result`] must be called to determine if an I/O error occurred during writing."] # [doc = ""] # [doc = " It is advisable to use a buffered writer like [`BufWriter`](std::io::BufWriter)"] # [doc = " instead of an unbuffered writer like [`File`](std::fs::File)."] # [cfg (feature = "std")] # [derive (Debug)] pub struct StreamingBuffer < W > { writer : W , len : usize , result : Result < () , io :: Error > , }
};
}
