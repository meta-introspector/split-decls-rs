// Generated macro for impl_15 (impl)
macro_rules! Depcrate_bfloatimpl_15 {
() => {
// Module: crate::bfloat
// Provides: {"impl_15"}
// Dependencies: {}
impl From < u8 > for bf16 { # [inline] fn from (x : u8) -> bf16 { bf16 :: from_f32 (f32 :: from (x)) } }
};
}
