// Generated macro for impl_1587 (impl)
macro_rules! Depcrate_sliceimpl_1587 {
() => {
// Module: crate::slice
// Provides: {"impl_1587"}
// Dependencies: {}
impl < 'data , T : Sync > ParallelIterator for Windows < 'data , T > { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
