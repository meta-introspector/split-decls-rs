// Generated macro for impl_24 (impl)
macro_rules! Depcrate_block_sse2impl_24 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_24"}
// Dependencies: {}
impl Not for Block { type Output = Block ; # [inline] fn not (self) -> Self :: Output { unsafe { Self (_mm_xor_si128 (self . 0 , Self :: ALL . 0)) } } }
};
}
