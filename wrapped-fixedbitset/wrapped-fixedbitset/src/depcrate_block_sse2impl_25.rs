// Generated macro for impl_25 (impl)
macro_rules! Depcrate_block_sse2impl_25 {
() => {
// Module: crate::block::sse2
// Provides: {"impl_25"}
// Dependencies: {}
impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { unsafe { Self (_mm_and_si128 (self . 0 , other . 0)) } } }
};
}
