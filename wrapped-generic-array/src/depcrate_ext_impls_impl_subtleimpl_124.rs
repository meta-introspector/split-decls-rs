// Generated macro for impl_124 (impl)
macro_rules! Depcrate_ext_impls_impl_subtleimpl_124 {
() => {
// Module: crate::ext_impls::impl_subtle
// Provides: {"impl_124"}
// Dependencies: {}
impl < T , N : ArrayLength > ConstantTimeEq for GenericArray < T , N > where T : ConstantTimeEq , { # [inline] fn ct_eq (& self , other : & Self) -> subtle :: Choice { self . as_slice () . ct_eq (other . as_slice ()) } }
};
}
