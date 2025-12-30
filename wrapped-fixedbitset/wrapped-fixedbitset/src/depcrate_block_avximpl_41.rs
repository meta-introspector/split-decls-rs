// Generated macro for impl_41 (impl)
macro_rules! Depcrate_block_avximpl_41 {
() => {
// Module: crate::block::avx
// Provides: {"impl_41"}
// Dependencies: {}
impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { unsafe { self . 0 = _mm256_and_pd (self . 0 , other . 0) ; } } }
};
}
