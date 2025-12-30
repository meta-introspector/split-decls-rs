// Generated macro for impl_47 (impl)
macro_rules! Depcrate_implsimpl_47 {
() => {
// Module: crate::impls
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : Ord , N : ArrayLength > Ord for GenericArray < T , N > { # [inline (always)] fn cmp (& self , other : & GenericArray < T , N >) -> Ordering { Ord :: cmp (self . as_slice () , other . as_slice ()) } }
};
}
