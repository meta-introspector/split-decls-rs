// Generated macro for impl_431 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_431 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_431"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , A : Allocator + Clone > IntoIterator for BTreeMap < K , V , A > { type Item = (K , V) ; type IntoIter = IntoIter < K , V , A > ; # [doc = " Gets an owning iterator over the entries of the map, sorted by key."] fn into_iter (self) -> IntoIter < K , V , A > { let mut me = ManuallyDrop :: new (self) ; if let Some (root) = me . root . take () { let full_range = root . into_dying () . full_range () ; IntoIter { range : full_range , length : me . length , alloc : unsafe { ManuallyDrop :: take (& mut me . alloc) } , } } else { IntoIter { range : LazyLeafRange :: none () , length : 0 , alloc : unsafe { ManuallyDrop :: take (& mut me . alloc) } , } } } }
};
}
