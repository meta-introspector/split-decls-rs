// Generated macro for impl_415 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_415 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_415"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < K : fmt :: Debug , V : fmt :: Debug > fmt :: Debug for RangeMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let range = Range { inner : self . inner . reborrow () } ; f . debug_list () . entries (range) . finish () } }
};
}
