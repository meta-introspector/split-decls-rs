// Generated macro for impl_266 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_266 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_266"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : Ord + fmt :: Debug , A : Allocator > fmt :: Debug for PeekMut < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("PeekMut") . field (& self . heap . data [0]) . finish () } }
};
}
