// Generated macro for impl_1498 (impl)
macro_rules! Depcrate_slice_rchunksimpl_1498 {
() => {
// Module: crate::slice::rchunks
// Provides: {"impl_1498"}
// Dependencies: {}
impl < 'data , T : Send > ParallelIterator for RChunksMut < 'data , T > { type Item = & 'data mut [T] ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
