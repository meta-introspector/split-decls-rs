// Generated macro for impl_55 (impl)
macro_rules! Depcrate_block_avx2impl_55 {
() => {
// Module: crate::block::avx2
// Provides: {"impl_55"}
// Dependencies: {}
impl BitAnd for Block { type Output = Block ; # [inline] fn bitand (self , other : Self) -> Self :: Output { unsafe { Self (_mm256_and_si256 (self . 0 , other . 0)) } } }
};
}
