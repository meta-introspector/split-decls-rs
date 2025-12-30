// Generated macro for impl_213 (impl)
macro_rules! Depcrate_streamimpl_213 {
() => {
// Module: crate::stream
// Provides: {"impl_213"}
// Dependencies: {}
impl < S : Stream + Unpin + ? Sized > Stream for Drain < '_ , S > { type Item = S :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . stream . poll_next (cx) { Poll :: Ready (x) => Poll :: Ready (x) , Poll :: Pending => Poll :: Ready (None) , } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , hi) = self . stream . size_hint () ; (0 , hi) } }
};
}
