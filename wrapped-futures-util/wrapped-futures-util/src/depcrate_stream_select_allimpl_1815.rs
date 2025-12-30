// Generated macro for impl_1815 (impl)
macro_rules! Depcrate_stream_select_allimpl_1815 {
() => {
// Module: crate::stream::select_all
// Provides: {"impl_1815"}
// Dependencies: {}
impl < 'a , St : Stream + Unpin > Iterator for IterMut < 'a , St > { type Item = & 'a mut St ; fn next (& mut self) -> Option < Self :: Item > { let st = self . 0 . next () ? ; let next = st . get_mut () ; debug_assert ! (next . is_some ()) ; next } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
