// Generated macro for impl_409 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_409 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_409"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K : fmt :: Debug , V , A : Allocator + Clone > fmt :: Debug for IntoKeys < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (key , _) | key)) . finish () } }
};
}
