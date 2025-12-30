// Generated macro for impl_29 (impl)
macro_rules! Depcrate_block_sse2impl_29 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_29"}
// Dependencies: {}
impl BitXor for Block { type Output = Block ; # [inline] fn bitxor (self , other : Self) -> Self :: Output { unsafe { Self (_mm_xor_si128 (self . 0 , other . 0)) } } }
};
}
