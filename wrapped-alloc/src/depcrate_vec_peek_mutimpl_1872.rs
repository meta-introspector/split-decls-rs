// Generated macro for impl_1872 (impl)
macro_rules! Depcrate_vec_peek_mutimpl_1872 {
() => {
// Module: crate::vec::peek_mut
// Provides: {"impl_1872"}
// Dependencies: {}
# [unstable (feature = "vec_peek_mut" , issue = "122742")] impl < T : fmt :: Debug > fmt :: Debug for PeekMut < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("PeekMut") . field (self . deref ()) . finish () } }
};
}
