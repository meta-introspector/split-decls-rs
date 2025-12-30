// Generated macro for impl_292 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_292 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_292"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . iter . as_slice ()) . finish () } }
};
}
