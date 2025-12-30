// Generated macro for impl_763 (impl)
macro_rules! Depcrate_collections_btree_setimpl_763 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_763"}
// Dependencies: {}
# [stable (feature = "btree_range" , since = "1.17.0")] impl < 'a , T > DoubleEndedIterator for Range < 'a , T > { fn next_back (& mut self) -> Option < & 'a T > { self . iter . next_back () . map (| (k , _) | k) } }
};
}
