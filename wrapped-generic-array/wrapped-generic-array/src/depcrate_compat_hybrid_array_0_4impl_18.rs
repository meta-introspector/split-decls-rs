// Generated macro for impl_18 (impl)
macro_rules! Depcrate_compat_hybrid_array_0_4impl_18 {
() => {
// Module: crate::compat::hybrid_array_0_4
// Provides: {"impl_18"}
// Dependencies: {}
impl < T , N : ArrayLength + ArraySize > From < HybridArray < T , N > > for GenericArray < T , N > { # [inline (always)] fn from (value : HybridArray < T , N >) -> Self { GenericArray :: from_ha0_4 (value) } }
};
}
