// Generated macro for impl_755 (impl)
macro_rules! Depcrate_collections_btree_setimpl_755 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_755"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator + Clone > Iterator for IntoIter < T , A > { type Item = T ; fn next (& mut self) -> Option < T > { self . iter . next () . map (| (k , _) | k) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
