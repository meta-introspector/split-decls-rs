// Generated macro for impl_400 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_400 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_400"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < K : Debug , V : Debug , A : Allocator + Clone > Debug for IntoIter < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
