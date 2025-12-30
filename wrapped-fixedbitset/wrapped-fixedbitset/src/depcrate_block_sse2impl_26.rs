// Generated macro for impl_26 (impl)
macro_rules! Depcrate_block_sse2impl_26 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_26"}
// Dependencies: {}
impl BitAndAssign for Block { # [inline] fn bitand_assign (& mut self , other : Self) { unsafe { self . 0 = _mm_and_si128 (self . 0 , other . 0) ; } } }
};
}
