// Generated macro for impl_488 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_488 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_488"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K : Ord , V : Ord , A : Allocator + Clone > Ord for BTreeMap < K , V , A > { # [inline] fn cmp (& self , other : & BTreeMap < K , V , A >) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
