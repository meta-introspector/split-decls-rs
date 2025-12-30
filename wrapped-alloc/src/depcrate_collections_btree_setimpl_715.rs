// Generated macro for impl_715 (impl)
macro_rules! Depcrate_collections_btree_setimpl_715 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_715"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug > fmt :: Debug for Iter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Iter") . field (& self . iter) . finish () } }
};
}
