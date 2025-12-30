// Generated macro for impl_43 (impl)
macro_rules! Depcrate_block_avximpl_43 {
() => {
// Module: crate::block::avx
// Provides: {"impl_43"}
// Dependencies: {}
impl BitOrAssign for Block { # [inline] fn bitor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_or_pd (self . 0 , other . 0) ; } } }
};
}
