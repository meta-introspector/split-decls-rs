// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_stream_try_stream_try_nextimpl_1241 {
() => {
// Module: crate::stream::try_stream::try_next
// Provides: {"impl_1241"}
// Dependencies: {}
impl < St : ? Sized + TryStream + Unpin > Future for TryNext < '_ , St > { type Output = Result < Option < St :: Ok > , St :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . stream . try_poll_next_unpin (cx) ? . map (Ok) } }
};
}
