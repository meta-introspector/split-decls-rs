// Generated macro for impl_403 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_403 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_403"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < K : fmt :: Debug , V > fmt :: Debug for Keys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
