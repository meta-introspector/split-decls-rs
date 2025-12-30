// Generated macro for impl_205 (impl)
macro_rules! Depcrate_vec_into_iterimpl_205 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_205"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
};
}
