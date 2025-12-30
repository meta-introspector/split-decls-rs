// Generated macro for impl_19 (impl)
macro_rules! Depcrate_compat_hybrid_array_0_4impl_19 {
() => {
// Module: crate::compat::hybrid_array_0_4
// Provides: {"impl_19"}
// Dependencies: {}
impl < T , N : ArrayLength + ArraySize > From < GenericArray < T , N > > for HybridArray < T , N > { # [inline (always)] fn from (value : GenericArray < T , N >) -> Self { value . into_ha0_4 () } }
};
}
