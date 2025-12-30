// Generated macro for impl_86 (impl)
macro_rules! Depcrate_ext_impls_impl_allocimpl_86 {
() => {
// Module: crate::ext_impls::impl_alloc
// Provides: {"impl_86"}
// Dependencies: {}
impl < T , N : ArrayLength > From < GenericArray < T , N > > for Box < [T] > { # [inline] fn from (value : GenericArray < T , N >) -> Self { Box :: new (value) . into_boxed_slice () } }
};
}
