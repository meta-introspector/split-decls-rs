// Generated macro for impl_139 (impl)
macro_rules! Depcrate_ext_impls_impl_as_sliceimpl_139 {
() => {
// Module: crate::ext_impls::impl_as_slice
// Provides: {"impl_139"}
// Dependencies: {}
impl < T , N : ArrayLength > AsMutSlice for GenericArray < T , N > { # [inline (always)] fn as_mut_slice (& mut self) -> & mut [T] { self . as_mut () } }
};
}
