// Generated macro for impl_487 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_487 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_487"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K : PartialOrd , V : PartialOrd , A : Allocator + Clone > PartialOrd for BTreeMap < K , V , A > { # [inline] fn partial_cmp (& self , other : & BTreeMap < K , V , A >) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
