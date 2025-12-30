// Generated macro for impl_87 (impl)
macro_rules! Depcrate_ext_impls_impl_allocimpl_87 {
() => {
// Module: crate::ext_impls::impl_alloc
// Provides: {"impl_87"}
// Dependencies: {}
impl < T , N : ArrayLength > From < GenericArray < T , N > > for Vec < T > { # [inline] fn from (value : GenericArray < T , N >) -> Self { Box :: < [T] > :: from (value) . into () } }
};
}
