// Generated macro for impl_59 (impl)
macro_rules! Depcrate_bfloatimpl_59 {
() => {
// Module: crate::bfloat
// Provides: {"impl_59"}
// Dependencies: {}
impl Product for bf16 { # [inline] fn product < I : Iterator < Item = Self > > (iter : I) -> Self { bf16 :: from_f32 (iter . map (| f | f . to_f32 ()) . product ()) } }
};
}
