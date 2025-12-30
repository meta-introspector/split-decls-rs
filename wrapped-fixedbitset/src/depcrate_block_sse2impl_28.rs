// Generated macro for impl_28 (impl)
macro_rules! Depcrate_block_sse2impl_28 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_28"}
// Dependencies: {}
impl BitOrAssign for Block { # [inline] fn bitor_assign (& mut self , other : Self) { unsafe { self . 0 = _mm_or_si128 (self . 0 , other . 0) ; } } }
};
}
