// Generated macro for impl_422 (impl)
macro_rules! Depcrate_iter_indeximpl_422 {
() => {
// Module: crate::iter_index
// Provides: {"impl_422"}
// Dependencies: {}
impl < I > IteratorIndex < I > for RangeToInclusive < usize > where I : Iterator , { type Output = Take < I > ; fn index (self , iter : I) -> Self :: Output { assert_ne ! (self . end , usize :: MAX) ; iter . take (self . end + 1) } }
};
}
