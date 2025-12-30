// Generated macro for impl_39 (impl)
macro_rules! Depcrate_block_avximpl_39 {
() => {
// Module: crate::block::avx
// Provides: {"impl_39"}
// Dependencies: {}
impl Not for Block { type Output = Block ; # [inline] fn not (self) -> Self :: Output { unsafe { Self (_mm256_xor_pd (self . 0 , Self :: ALL . 0)) } } }
};
}
