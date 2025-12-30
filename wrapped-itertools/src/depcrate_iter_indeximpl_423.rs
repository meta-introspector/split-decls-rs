// Generated macro for impl_423 (impl)
macro_rules! Depcrate_iter_indeximpl_423 {
() => {
// Module: crate::iter_index
// Provides: {"impl_423"}
// Dependencies: {}
impl < I > IteratorIndex < I > for RangeFrom < usize > where I : Iterator , { type Output = Skip < I > ; fn index (self , iter : I) -> Self :: Output { iter . skip (self . start) } }
};
}
