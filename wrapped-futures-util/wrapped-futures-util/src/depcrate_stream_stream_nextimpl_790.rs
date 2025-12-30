// Generated macro for impl_790 (impl)
macro_rules! Depcrate_stream_stream_nextimpl_790 {
() => {
// Module: crate::stream::stream::next
// Provides: {"impl_790"}
// Dependencies: {}
impl < St : ? Sized + Stream + Unpin > Future for Next < '_ , St > { type Output = Option < St :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . stream . poll_next_unpin (cx) } }
};
}
