// Generated macro for impl_425 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_425 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_425"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , K , V , A : Allocator + Clone > IntoIterator for & 'a mut BTreeMap < K , V , A > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
};
}
