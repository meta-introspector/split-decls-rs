// Generated macro for impl_23 (impl)
macro_rules! Depcrate_compressorimpl_23 {
() => {
// Module: crate::compressor
// Provides: {"impl_23"}
// Dependencies: {}
impl BitXor for X8 { type Output = Self ; # [inline (always)] fn bitxor (self , rhs : Self) -> Self :: Output { (self , rhs) . map (| x , y | unsafe { _mm_xor_si128 (x , y) }) } }
};
}
