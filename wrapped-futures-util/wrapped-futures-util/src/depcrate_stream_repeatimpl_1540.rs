// Generated macro for impl_1540 (impl)
macro_rules! Depcrate_stream_repeatimpl_1540 {
() => {
// Module: crate::stream::repeat
// Provides: {"impl_1540"}
// Dependencies: {}
impl < T > Stream for Repeat < T > where T : Clone , { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (Some (self . item . clone ())) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
