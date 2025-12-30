// Generated macro for impl_1485 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1485 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1485"}
// Dependencies: {}
impl < 'data , T : Sync > ParallelIterator for RChunks < 'data , T > { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
