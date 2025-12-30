// Generated macro for impl_145 (impl)
macro_rules! Depcrate_recv_streamimpl_145 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_145"}
// Dependencies: {}
impl Future for ReadChunks < '_ > { type Output = Result < Option < usize > , ReadError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let this = self . get_mut () ; this . stream . poll_read_chunks (cx , this . bufs) } }
};
}
