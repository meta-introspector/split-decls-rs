// Generated macro for impl_41 (impl)
macro_rules! Depcrate_bfloatimpl_41 {
() => {
// Module: crate::bfloat
// Provides: {"impl_41"}
// Dependencies: {}
impl Mul for bf16 { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { Self :: from_f32 (Self :: to_f32 (self) * Self :: to_f32 (rhs)) } }
};
}
