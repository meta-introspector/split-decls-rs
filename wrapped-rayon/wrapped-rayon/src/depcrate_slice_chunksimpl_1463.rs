// Generated macro for impl_1463 (impl)
macro_rules! Depcrate_slice_chunksimpl_1463 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1463"}
// Dependencies: {}
impl < 'data , T : Sync > ParallelIterator for ChunksExact < 'data , T > { type Item = & 'data [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
