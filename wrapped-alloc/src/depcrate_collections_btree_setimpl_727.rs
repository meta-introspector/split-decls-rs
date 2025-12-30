// Generated macro for impl_727 (impl)
macro_rules! Depcrate_collections_btree_setimpl_727 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_727"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : Debug , A : Allocator + Clone > Debug for Intersection < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Intersection") . field (& self . inner) . finish () } }
};
}
