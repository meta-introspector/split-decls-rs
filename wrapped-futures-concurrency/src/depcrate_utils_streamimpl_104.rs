// Generated macro for impl_104 (impl)
macro_rules! Depcrate_utils_streamimpl_104 {
() => {
// Module: crate::utils::stream
// Provides: {"impl_104"}
// Dependencies: {}
impl < I : Iterator > Stream for FromIter < I > { type Item = I :: Item ; fn poll_next (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (self . iter . next ()) } }
};
}
