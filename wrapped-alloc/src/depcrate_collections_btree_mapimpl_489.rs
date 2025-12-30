// Generated macro for impl_489 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_489 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_489"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K : Debug , V : Debug , A : Allocator + Clone > Debug for BTreeMap < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter ()) . finish () } }
};
}
