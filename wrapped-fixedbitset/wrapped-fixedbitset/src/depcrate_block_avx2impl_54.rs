// Generated macro for impl_54 (impl)
macro_rules! Depcrate_block_avx2impl_54 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_54"}
// Dependencies: {}
impl Not for Block { type Output = Block ; # [inline] fn not (self) -> Self :: Output { unsafe { Self (_mm256_xor_si256 (self . 0 , Self :: ALL . 0)) } } }
};
}
