// Generated macro for impl_126 (impl)
macro_rules! Depcrate_binary16impl_126 {
() => {
// Module: crate::binary16
// Provides: {"impl_126"}
// Dependencies: {}
impl Sum for f16 { # [inline] fn sum < I : Iterator < Item = Self > > (iter : I) -> Self { f16 (arch :: sum_f16 (iter . map (| f | f . to_bits ()))) } }
};
}
