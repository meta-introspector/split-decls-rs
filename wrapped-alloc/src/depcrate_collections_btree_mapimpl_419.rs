// Generated macro for impl_419 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_419 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_419"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V , A : Allocator + Clone > IntoIterator for & 'a BTreeMap < K , V , A > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
};
}
