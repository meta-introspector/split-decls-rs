// Generated macro for impl_21 (impl)
macro_rules! Depcrate_compat_hybrid_array_0_4impl_21 {
() => {
// Module: crate::compat::hybrid_array_0_4
// Provides: {"impl_21"}
// Dependencies: {}
impl < T , N : ArrayLength + ArraySize > AsMut < HybridArray < T , N > > for GenericArray < T , N > { # [inline (always)] fn as_mut (& mut self) -> & mut HybridArray < T , N > { self . as_ha0_4_mut () } }
};
}
