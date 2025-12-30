// Generated macro for impl_1564 (impl)
macro_rules! Depcrate_stream_emptyimpl_1564 {
() => {
// Module: crate::stream::empty
// Provides: {"impl_1564"}
// Dependencies: {}
impl < T > Stream for Empty < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (None) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
