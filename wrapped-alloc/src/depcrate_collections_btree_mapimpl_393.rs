// Generated macro for impl_393 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_393 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_393"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for Iter < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
