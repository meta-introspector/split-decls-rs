// Generated macro for impl_17 (impl)
macro_rules! Depcrate_compat_hybrid_array_0_4impl_17 {
() => {
// Module: crate::compat::hybrid_array_0_4
// Provides: {"impl_17"}
// Dependencies: {}
impl < T , N : ArrayLength + ArraySize > AsArrayMut < T > for GenericArray < T , N > { # [inline (always)] fn as_array_mut (& mut self) -> & mut HybridArray < T , N > { self . as_ha0_4_mut () } }
};
}
