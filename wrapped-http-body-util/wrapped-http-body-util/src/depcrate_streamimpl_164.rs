// Generated macro for impl_164 (impl)
macro_rules! Depcrate_streamimpl_164 {
() => {
// Module: crate::stream
// Provides: {"impl_164"}
// Dependencies: {}
impl < S : Stream > Stream for StreamBody < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . project () . stream . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
};
}
