// Generated macro for impl_751 (impl)
macro_rules! Depcrate_collections_btree_setimpl_751 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_751"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } fn last (mut self) -> Option < & 'a T > { self . next_back () } fn min (mut self) -> Option < & 'a T > where & 'a T : Ord , { self . next () } fn max (mut self) -> Option < & 'a T > where & 'a T : Ord , { self . next_back () } }
};
}
