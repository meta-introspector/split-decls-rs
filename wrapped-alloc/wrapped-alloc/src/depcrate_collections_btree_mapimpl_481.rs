// Generated macro for impl_481 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_481 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_481"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K : Ord , V , A : Allocator + Clone > Extend < (K , V) > for BTreeMap < K , V , A > { # [inline] fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { iter . into_iter () . for_each (move | (k , v) | { self . insert (k , v) ; }) ; } # [inline] fn extend_one (& mut self , (k , v) : (K , V)) { self . insert (k , v) ; } }
};
}
