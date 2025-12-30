// Generated macro for impl_71 (impl)
macro_rules! Depcrate_iterimpl_71 {
() => {
// Module: crate::iter
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : fmt :: Debug , N : ArrayLength > fmt :: Debug for GenericArrayIter < T , N > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("GenericArrayIter") . field (& self . as_slice ()) . finish () } }
};
}
