// Generated macro for impl_483 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_483 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_483"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K : Hash , V : Hash , A : Allocator + Clone > Hash for BTreeMap < K , V , A > { fn hash < H : Hasher > (& self , state : & mut H) { state . write_length_prefix (self . len ()) ; for elt in self { elt . hash (state) ; } } }
};
}
