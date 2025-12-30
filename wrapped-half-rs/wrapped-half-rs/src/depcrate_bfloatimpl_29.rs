// Generated macro for impl_29 (impl)
macro_rules! Depcrate_bfloatimpl_29 {
() => {
// Module: crate::bfloat
// Provides: {"impl_29"}
// Dependencies: {}
impl Add for bf16 { type Output = Self ; fn add (self , rhs : Self) -> Self :: Output { Self :: from_f32 (Self :: to_f32 (self) + Self :: to_f32 (rhs)) } }
};
}
