// Generated macro for impl_53 (impl)
macro_rules! Depcrate_bfloatimpl_53 {
() => {
// Module: crate::bfloat
// Provides: {"impl_53"}
// Dependencies: {}
impl Rem for bf16 { type Output = Self ; fn rem (self , rhs : Self) -> Self :: Output { Self :: from_f32 (Self :: to_f32 (self) % Self :: to_f32 (rhs)) } }
};
}
