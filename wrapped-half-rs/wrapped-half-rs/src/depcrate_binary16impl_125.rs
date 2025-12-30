// Generated macro for impl_125 (impl)
macro_rules! Depcrate_binary16impl_125 {
() => {
// Module: crate::binary16
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'a > Product < & 'a f16 > for f16 { # [inline] fn product < I : Iterator < Item = & 'a f16 > > (iter : I) -> Self { f16 (arch :: product_f16 (iter . map (| f | f . to_bits ()))) } }
};
}
