// Generated macro for impl_46 (impl)
macro_rules! Depcrate_implsimpl_46 {
() => {
// Module: crate::impls
// Provides: {"impl_46"}
// Dependencies: {}
impl < T : PartialOrd , N : ArrayLength > PartialOrd for GenericArray < T , N > { # [inline (always)] fn partial_cmp (& self , other : & GenericArray < T , N >) -> Option < Ordering > { PartialOrd :: partial_cmp (self . as_slice () , other . as_slice ()) } }
};
}
