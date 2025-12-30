// Generated macro for impl_721 (impl)
macro_rules! Depcrate_collections_btree_setimpl_721 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_721"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug , A : Allocator + Clone > fmt :: Debug for Difference < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Difference") . field (& self . inner) . finish () } }
};
}
