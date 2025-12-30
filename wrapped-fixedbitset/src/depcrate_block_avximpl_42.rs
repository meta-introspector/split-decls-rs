// Generated macro for impl_42 (impl)
macro_rules! Depcrate_block_avximpl_42 {
() => {
// Module: crate::block::avx
// Provides: {"impl_42"}
// Dependencies: {}
impl BitOr for Block { type Output = Block ; # [inline] fn bitor (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_or_pd (self . 0 , other . 0)) } } }
};
}
