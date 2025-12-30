// Generated macro for impl_129 (impl)
macro_rules! Depcrate_recv_streamimpl_129 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_129"}
// Dependencies: {}
impl tokio :: io :: AsyncRead for RecvStream { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { ready ! (Self :: poll_read_buf (self . get_mut () , cx , buf)) ? ; Poll :: Ready (Ok (())) } }
};
}
