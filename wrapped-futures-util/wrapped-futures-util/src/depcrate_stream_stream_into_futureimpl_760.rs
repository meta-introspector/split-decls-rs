// Generated macro for impl_760 (impl)
macro_rules! Depcrate_stream_stream_into_futureimpl_760 {
() => {
// Module: crate::stream::stream::into_future
// Provides: {"impl_760"}
// Dependencies: {}
impl < St : Stream + Unpin > Future for StreamFuture < St > { type Output = (Option < St :: Item > , St) ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let item = { let s = self . stream . as_mut () . expect ("polling StreamFuture twice") ; ready ! (s . poll_next_unpin (cx)) } ; let stream = self . stream . take () . unwrap () ; Poll :: Ready ((item , stream)) } }
};
}
