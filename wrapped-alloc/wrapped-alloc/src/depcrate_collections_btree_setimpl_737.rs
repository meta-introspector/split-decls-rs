// Generated macro for impl_737 (impl)
macro_rules! Depcrate_collections_btree_setimpl_737 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_737"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator + Clone > IntoIterator for & 'a BTreeSet < T , A > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
