// Generated macro for impl_82 (impl)
macro_rules! Depcrate_streamimpl_82 {
() => {
// Module: crate::stream
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > Stream for Pending < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < T > > { Poll :: Pending } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
