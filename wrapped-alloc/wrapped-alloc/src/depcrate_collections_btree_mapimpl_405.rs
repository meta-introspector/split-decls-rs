// Generated macro for impl_405 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_405 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_405"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < K , V : fmt :: Debug > fmt :: Debug for Values < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
