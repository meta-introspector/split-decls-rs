// Generated macro for ReadChunk (struct)
macro_rules! Depcrate_recv_streamReadChunk {
() => {
// Module: crate::recv_stream
// Provides: {"ReadChunk"}
// Dependencies: {}
# [doc = " Future produced by [`RecvStream::read_chunk()`]."] # [doc = ""] # [doc = " [`RecvStream::read_chunk()`]: crate::RecvStream::read_chunk"] struct ReadChunk < 'a > { stream : & 'a mut RecvStream , max_length : usize , ordered : bool , }
};
}
