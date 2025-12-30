// Generated macro for impl_61 (impl)
macro_rules! Depcrate_bfloatimpl_61 {
() => {
// Module: crate::bfloat
// Provides: {"impl_61"}
// Dependencies: {}
impl Sum for bf16 { # [inline] fn sum < I : Iterator < Item = Self > > (iter : I) -> Self { bf16 :: from_f32 (iter . map (| f | f . to_f32 ()) . sum ()) } }
};
}
