// Generated macro for impl_284 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_284 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_284"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug > fmt :: Debug for Iter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Iter") . field (& self . iter . as_slice ()) . finish () } }
};
}
