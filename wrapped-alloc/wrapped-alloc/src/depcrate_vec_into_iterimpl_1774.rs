// Generated macro for impl_1774 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1774 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1774"}
// Dependencies: {}
# [stable (feature = "vec_intoiter_debug" , since = "1.13.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
};
}
