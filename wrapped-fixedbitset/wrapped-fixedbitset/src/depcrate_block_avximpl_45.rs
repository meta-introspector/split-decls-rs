// Generated macro for impl_45 (impl)
macro_rules! Depcrate_block_avximpl_45 {
() => {
// Module: crate::block::avx
// Provides: {"impl_45"}
// Dependencies: {}
impl BitXorAssign for Block { # [inline] fn bitxor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_xor_pd (self . 0 , other . 0) } } }
};
}
