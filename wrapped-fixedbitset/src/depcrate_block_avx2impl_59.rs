// Generated macro for impl_59 (impl)
macro_rules! Depcrate_block_avx2impl_59 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_59"}
// Dependencies: {}
impl BitXor for Block { type Output = Block ; # [inline] fn bitxor (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_xor_si256 (self . 0 , other . 0)) } } }
};
}
