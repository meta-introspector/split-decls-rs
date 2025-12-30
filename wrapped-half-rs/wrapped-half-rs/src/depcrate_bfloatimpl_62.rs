// Generated macro for impl_62 (impl)
macro_rules! Depcrate_bfloatimpl_62 {
() => {
// Module: crate::bfloat
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a > Sum < & 'a bf16 > for bf16 { # [inline] fn sum < I : Iterator < Item = & 'a bf16 > > (iter : I) -> Self { bf16 :: from_f32 (iter . map (| f | f . to_f32 ()) . sum ()) } }
};
}
