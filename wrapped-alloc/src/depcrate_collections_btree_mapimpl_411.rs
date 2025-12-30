// Generated macro for impl_411 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_411 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_411"}
// Dependencies: {}
# [stable (feature = "map_into_keys_values" , since = "1.54.0")] impl < K , V : fmt :: Debug , A : Allocator + Clone > fmt :: Debug for IntoValues < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . inner . iter () . map (| (_ , val) | val)) . finish () } }
};
}
