// Generated macro for impl_127 (impl)
macro_rules! Depcrate_binary16impl_127 {
() => {
// Module: crate::binary16
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a > Sum < & 'a f16 > for f16 { # [inline] fn sum < I : Iterator < Item = & 'a f16 > > (iter : I) -> Self { f16 (arch :: sum_f16 (iter . map (| f | f . to_bits ()))) } }
};
}
