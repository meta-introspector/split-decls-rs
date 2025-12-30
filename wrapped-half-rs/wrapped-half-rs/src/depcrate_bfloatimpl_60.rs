// Generated macro for impl_60 (impl)
macro_rules! Depcrate_bfloatimpl_60 {
() => {
// Module: crate::bfloat
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a > Product < & 'a bf16 > for bf16 { # [inline] fn product < I : Iterator < Item = & 'a bf16 > > (iter : I) -> Self { bf16 :: from_f32 (iter . map (| f | f . to_f32 ()) . product ()) } }
};
}
