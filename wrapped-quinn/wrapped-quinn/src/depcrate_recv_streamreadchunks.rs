// Generated macro for ReadChunks (struct)
macro_rules! Depcrate_recv_streamReadChunks {
() => {
// Module: crate::recv_stream
// Provides: {"ReadChunks"}
// Dependencies: {}
# [doc = " Future produced by [`RecvStream::read_chunks()`]."] # [doc = ""] # [doc = " [`RecvStream::read_chunks()`]: crate::RecvStream::read_chunks"] struct ReadChunks < 'a > { stream : & 'a mut RecvStream , bufs : & 'a mut [Bytes] , }
};
}
