// Generated macro for impl_138 (impl)
macro_rules! Depcrate_ext_impls_impl_as_sliceimpl_138 {
() => {
// Module: crate::ext_impls::impl_as_slice
// Provides: {"impl_138"}
// Dependencies: {}
impl < T , N : ArrayLength > AsSlice for GenericArray < T , N > { type Element = T ; # [inline (always)] fn as_slice (& self) -> & [T] { self . as_ref () } }
};
}
