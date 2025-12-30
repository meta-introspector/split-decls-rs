// Generated macro for impl_58 (impl)
macro_rules! Depcrate_block_avx2impl_58 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_58"}
// Dependencies: {}
impl BitOrAssign for Block { # [inline] fn bitor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_or_si256 (self . 0 , other . 0) ; } } }
};
}
