// Generated macro for impl_44 (impl)
macro_rules! Depcrate_block_avximpl_44 {
() => {
// Module: crate::block::avx
// Provides: {"impl_44"}
// Dependencies: {}
impl BitXor for Block { type Output = Block ; # [inline] fn bitxor (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_xor_pd (self . 0 , other . 0)) } } }
};
}
