// Generated macro for impl_47 (impl)
macro_rules! Depcrate_bfloatimpl_47 {
() => {
// Module: crate::bfloat
// Provides: {"impl_47"}
// Dependencies: {}
impl Div for bf16 { type Output = Self ; fn div (self , rhs : Self) -> Self :: Output { Self :: from_f32 (Self :: to_f32 (self) / Self :: to_f32 (rhs)) } }
};
}
