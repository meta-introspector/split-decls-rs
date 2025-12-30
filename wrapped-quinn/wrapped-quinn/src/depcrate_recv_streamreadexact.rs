// Generated macro for ReadExact (struct)
macro_rules! Depcrate_recv_streamReadExact {
() => {
// Module: crate::recv_stream
// Provides: {"ReadExact"}
// Dependencies: {}
# [doc = " Future produced by [`RecvStream::read_exact()`]."] # [doc = ""] # [doc = " [`RecvStream::read_exact()`]: crate::RecvStream::read_exact"] struct ReadExact < 'a > { stream : & 'a mut RecvStream , buf : ReadBuf < 'a > , }
};
}
