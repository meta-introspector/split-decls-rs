// Generated macro for impl_1475 (impl)
macro_rules! Depcrate_slice_chunksimpl_1475 {
() => {
// Module: crate::slice::chunks
// Provides: {"impl_1475"}
// Dependencies: {}
impl < 'data , T : Send > ParallelIterator for ChunksExactMut < 'data , T > { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
