// Generated macro for impl_757 (impl)
macro_rules! Depcrate_collections_btree_setimpl_757 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_757"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator + Clone > DoubleEndedIterator for IntoIter < T , A > { fn next_back (& mut self) -> Option < T > { self . iter . next_back () . map (| (k , _) | k) } }
};
}
