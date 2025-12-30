// Generated macro for impl_1727 (impl)
macro_rules! Depcrate_stream_futures_unordered_iterimpl_1727 {
() => {
// Module: crate::stream::futures_unordered::iter
// Provides: {"impl_1727"}
// Dependencies: {}
impl < 'a , Fut : Unpin > Iterator for IterMut < 'a , Fut > { type Item = & 'a mut Fut ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (Pin :: get_mut) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
