// Generated macro for impl_1731 (impl)
macro_rules! Depcrate_stream_futures_unordered_iterimpl_1731 {
() => {
// Module: crate::stream::futures_unordered::iter
// Provides: {"impl_1731"}
// Dependencies: {}
impl < 'a , Fut : Unpin > Iterator for Iter < 'a , Fut > { type Item = & 'a Fut ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (Pin :: get_ref) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
