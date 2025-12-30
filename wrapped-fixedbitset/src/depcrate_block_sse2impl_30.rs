// Generated macro for impl_30 (impl)
macro_rules! Depcrate_block_sse2impl_30 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_30"}
// Dependencies: {}
impl BitXorAssign for Block { # [inline] fn bitxor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm_xor_si128 (self . 0 , other . 0) } } }
};
}
