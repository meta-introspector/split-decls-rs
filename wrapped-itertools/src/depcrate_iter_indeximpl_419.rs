// Generated macro for impl_419 (impl)
macro_rules! Depcrate_iter_indeximpl_419 {
() => {
// Module: crate::iter_index
// Provides: {"impl_419"}
// Dependencies: {}
impl < I > IteratorIndex < I > for Range < usize > where I : Iterator , { type Output = Skip < Take < I > > ; fn index (self , iter : I) -> Self :: Output { iter . take (self . end) . skip (self . start) } }
};
}
