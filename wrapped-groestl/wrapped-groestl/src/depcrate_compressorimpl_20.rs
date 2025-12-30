// Generated macro for impl_20 (impl)
macro_rules! Depcrate_compressorimpl_20 {
() => {
// Module: crate::compressor
// Provides: {"impl_20"}
// Dependencies: {}
impl BitXor for X4 { type Output = Self ; # [inline (always)] fn bitxor (self , rhs : Self) -> Self :: Output { (self , rhs) . map (| x , y | unsafe { _mm_xor_si128 (x , y) }) } }
};
}
