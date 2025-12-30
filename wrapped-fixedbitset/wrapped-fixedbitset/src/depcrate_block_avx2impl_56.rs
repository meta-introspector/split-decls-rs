// Generated macro for impl_56 (impl)
macro_rules! Depcrate_block_avx2impl_56 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_56"}
// Dependencies: {}
impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_and_si256 (self . 0 , other . 0) ; } } }
};
}
