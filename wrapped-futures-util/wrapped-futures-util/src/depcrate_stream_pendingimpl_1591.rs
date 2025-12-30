// Generated macro for impl_1591 (impl)
macro_rules! Depcrate_stream_pendingimpl_1591 {
() => {
// Module: crate::stream::pending
// Provides: {"impl_1591"}
// Dependencies: {}
impl < T > Stream for Pending < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Pending } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}
