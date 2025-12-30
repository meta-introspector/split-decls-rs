// Generated macro for impl_117 (impl)
macro_rules! Depcrate_streamimpl_117 {
() => {
// Module: crate::stream
// Provides: {"impl_117"}
// Dependencies: {}
impl < S : Stream + Unpin + ? Sized > Future for NextFuture < '_ , S > { type Output = Option < S :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . stream . poll_next (cx) } }
};
}
