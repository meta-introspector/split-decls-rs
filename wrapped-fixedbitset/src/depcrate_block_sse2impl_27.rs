// Generated macro for impl_27 (impl)
macro_rules! Depcrate_block_sse2impl_27 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_27"}
// Dependencies: {}
impl BitOr for Block { type Output = Block ; # [inline] fn bitor (self , other : Self) -> Self :: Output { unsafe { Self (_mm_or_si128 (self . 0 , other . 0)) } } }
};
}
