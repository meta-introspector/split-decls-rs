// Generated macro for impl_749 (impl)
macro_rules! Depcrate_collections_btree_setimpl_749 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_749"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Debug , A : Allocator + Clone > Debug for BTreeSet < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
