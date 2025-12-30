// Generated macro for impl_958 (impl)
macro_rules! Depcrate_collections_vec_deque_iterimpl_958 {
() => {
// Module: crate::collections::vec_deque::iter
// Provides: {"impl_958"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug > fmt :: Debug for Iter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Iter") . field (& self . i1 . as_slice ()) . field (& self . i2 . as_slice ()) . finish () } }
};
}
