// Generated macro for impl_91 (impl)
macro_rules! Depcrate_streamimpl_91 {
() => {
// Module: crate::stream
// Provides: {"impl_91"}
// Dependencies: {}
impl < T : Clone > Stream for Repeat < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (Some (self . item . clone ())) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
