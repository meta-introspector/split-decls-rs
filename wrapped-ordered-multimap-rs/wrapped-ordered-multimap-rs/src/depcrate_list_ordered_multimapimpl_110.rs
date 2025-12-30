// Generated macro for impl_110 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_110 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'map , Key > Iterator for Keys < 'map , Key > { type Item = & 'map Key ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
