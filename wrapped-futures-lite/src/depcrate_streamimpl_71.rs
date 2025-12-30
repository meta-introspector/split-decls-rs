// Generated macro for impl_71 (impl)
macro_rules! Depcrate_streamimpl_71 {
() => {
// Module: crate::stream
// Provides: {"impl_71"}
// Dependencies: {}
impl < T > Stream for Empty < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (None) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
