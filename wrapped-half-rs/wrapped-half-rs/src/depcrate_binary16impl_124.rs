// Generated macro for impl_124 (impl)
macro_rules! Depcrate_binary16impl_124 {
() => {
// Module: crate::binary16
// Provides: {"impl_124"}
// Dependencies: {}
impl Product for f16 { # [inline] fn product < I : Iterator < Item = Self > > (iter : I) -> Self { f16 (arch :: product_f16 (iter . map (| f | f . to_bits ()))) } }
};
}
