// Generated macro for impl_16 (impl)
macro_rules! Depcrate_compat_hybrid_array_0_4impl_16 {
() => {
// Module: crate::compat::hybrid_array_0_4
// Provides: {"impl_16"}
// Dependencies: {}
impl < T , N : ArrayLength + ArraySize > AsArrayRef < T > for GenericArray < T , N > { # [inline (always)] fn as_array_ref (& self) -> & HybridArray < T , N > { self . as_ha0_4 () } }
};
}
