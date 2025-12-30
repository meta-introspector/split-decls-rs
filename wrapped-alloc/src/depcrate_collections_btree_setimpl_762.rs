// Generated macro for impl_762 (impl)
macro_rules! Depcrate_collections_btree_setimpl_762 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_762"}
// Dependencies: {}
# [stable (feature = "btree_range" , since = "1.17.0")] impl < 'a , T > Iterator for Range < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . iter . next () . map (| (k , _) | k) } fn last (mut self) -> Option < & 'a T > { self . next_back () } fn min (mut self) -> Option < & 'a T > where & 'a T : Ord , { self . next () } fn max (mut self) -> Option < & 'a T > where & 'a T : Ord , { self . next_back () } }
};
}
