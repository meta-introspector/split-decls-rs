// Generated macro for impl_20 (impl)
macro_rules! Depcrate_compat_hybrid_array_0_4impl_20 {
() => {
// Module: crate::compat::hybrid_array_0_4
// Provides: {"impl_20"}
// Dependencies: {}
impl < T , N : ArrayLength + ArraySize > AsRef < HybridArray < T , N > > for GenericArray < T , N > { # [inline (always)] fn as_ref (& self) -> & HybridArray < T , N > { self . as_ha0_4 () } }
};
}
