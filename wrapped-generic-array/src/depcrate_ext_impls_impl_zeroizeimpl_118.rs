// Generated macro for impl_118 (impl)
macro_rules! Depcrate_ext_impls_impl_zeroizeimpl_118 {
() => {
// Module: crate::ext_impls::impl_zeroize
// Provides: {"impl_118"}
// Dependencies: {}
impl < Z : Zeroize , N : ArrayLength > Zeroize for GenericArray < Z , N > { fn zeroize (& mut self) { self . as_mut_slice () . iter_mut () . zeroize () } }
};
}
