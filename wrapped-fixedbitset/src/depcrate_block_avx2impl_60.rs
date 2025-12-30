// Generated macro for impl_60 (impl)
macro_rules! Depcrate_block_avx2impl_60 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_60"}
// Dependencies: {}
impl BitXorAssign for Block { # [inline] fn bitxor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_xor_si256 (self . 0 , other . 0) } } }
};
}
