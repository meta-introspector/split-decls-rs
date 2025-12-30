// Generated macro for impl_1295 (impl)
macro_rules! Depcrate_iter_zip_eqimpl_1295 {
() => {
// Module: crate::iter::zip_eq
// Provides: {"impl_1295"}
// Dependencies: {}
impl < A , B > ParallelIterator for ZipEq < A , B > where A : IndexedParallelIterator , B : IndexedParallelIterator , { type Item = (A :: Item , B :: Item) ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self . zip , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . zip . len ()) } }
};
}
