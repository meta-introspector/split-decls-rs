// Generated macro for impl_75 (impl)
macro_rules! Depcrate_streamimpl_75 {
() => {
// Module: crate::stream
// Provides: {"impl_75"}
// Dependencies: {}
impl < I : Iterator > Stream for Iter < I > { type Item = I :: Item ; fn poll_next (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (self . iter . next ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
