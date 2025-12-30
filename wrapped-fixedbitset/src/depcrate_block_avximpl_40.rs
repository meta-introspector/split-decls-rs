// Generated macro for impl_40 (impl)
macro_rules! Depcrate_block_avximpl_40 {
() => {
// Module: crate::block::avx
// Provides: {"impl_40"}
// Dependencies: {}
impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_and_pd (self . 0 , other . 0)) } } }
};
}
