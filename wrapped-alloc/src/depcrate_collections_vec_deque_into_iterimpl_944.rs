// Generated macro for impl_944 (impl)
macro_rules! Depcrate_collections_vec_deque_into_iterimpl_944 {
() => {
// Module: crate::collections::vec_deque::into_iter
// Provides: {"impl_944"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . inner) . finish () } }
};
}
