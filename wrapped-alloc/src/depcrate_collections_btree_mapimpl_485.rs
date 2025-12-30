// Generated macro for impl_485 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_485 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_485"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K : PartialEq , V : PartialEq , A : Allocator + Clone > PartialEq for BTreeMap < K , V , A > { fn eq (& self , other : & BTreeMap < K , V , A >) -> bool { self . len () == other . len () && self . iter () . zip (other) . all (| (a , b) | a == b) } }
};
}
