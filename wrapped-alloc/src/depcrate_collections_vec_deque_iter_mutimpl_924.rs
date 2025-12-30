// Generated macro for impl_924 (impl)
macro_rules! Depcrate_collections_vec_deque_iter_mutimpl_924 {
() => {
// Module: crate::collections::vec_deque::iter_mut
// Provides: {"impl_924"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug > fmt :: Debug for IterMut < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IterMut") . field (& self . i1 . as_slice ()) . field (& self . i2 . as_slice ()) . finish () } }
};
}
