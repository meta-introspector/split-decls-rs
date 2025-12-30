// Generated macro for Read (struct)
macro_rules! Depcrate_recv_streamRead {
() => {
// Module: crate::recv_stream
// Provides: {"Read"}
// Dependencies: {}
# [doc = " Future produced by [`RecvStream::read()`]."] # [doc = ""] # [doc = " [`RecvStream::read()`]: crate::RecvStream::read"] struct Read < 'a > { stream : & 'a mut RecvStream , buf : ReadBuf < 'a > , }
};
}
