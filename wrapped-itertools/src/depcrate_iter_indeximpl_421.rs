// Generated macro for impl_421 (impl)
macro_rules! Depcrate_iter_indeximpl_421 {
() => {
// Module: crate::iter_index
// Provides: {"impl_421"}
// Dependencies: {}
impl < I > IteratorIndex < I > for RangeTo < usize > where I : Iterator , { type Output = Take < I > ; fn index (self , iter : I) -> Self :: Output { iter . take (self . end) } }
};
}
