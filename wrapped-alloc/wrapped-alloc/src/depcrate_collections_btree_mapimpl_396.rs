// Generated macro for impl_396 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_396 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_396"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for IterMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let range = Iter { range : self . range . reborrow () , length : self . length } ; f . debug_list () . entries (range) . finish () } }
};
}
