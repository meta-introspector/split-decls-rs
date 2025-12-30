// Generated macro for impl_35 (impl)
macro_rules! Depcrate_bfloatimpl_35 {
() => {
// Module: crate::bfloat
// Provides: {"impl_35"}
// Dependencies: {}
impl Sub for bf16 { type Output = Self ; fn sub (self , rhs : Self) -> Self :: Output { Self :: from_f32 (Self :: to_f32 (self) - Self :: to_f32 (rhs)) } }
};
}
