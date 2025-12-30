// Generated macro for impl_57 (impl)
macro_rules! Depcrate_block_avx2impl_57 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_57"}
// Dependencies: {}
impl BitOr for Block { type Output = Block ; # [inline] fn bitor (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_or_si256 (self . 0 , other . 0)) } } }
};
}
