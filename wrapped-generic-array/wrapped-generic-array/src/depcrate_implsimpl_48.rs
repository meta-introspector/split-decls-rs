// Generated macro for impl_48 (impl)
macro_rules! Depcrate_implsimpl_48 {
() => {
// Module: crate::impls
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : Debug , N : ArrayLength > Debug for GenericArray < T , N > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { self . as_slice () . fmt (fmt) } }
};
}
