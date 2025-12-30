// Generated macro for impl_1817 (impl)
macro_rules! Depcrate_stream_select_allimpl_1817 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1817"}
// Dependencies: {}
impl < St : Stream + Unpin > Iterator for IntoIter < St > { type Item = St ; fn next (& mut self) -> Option < Self :: Item > { let st = self . 0 . next () ? ; let next = st . into_inner () ; debug_assert ! (next . is_some ()) ; next } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
