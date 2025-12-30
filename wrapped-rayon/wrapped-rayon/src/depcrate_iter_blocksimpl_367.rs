// Generated macro for impl_367 (impl)
macro_rules! Depcrate_iter_blocksimpl_367 {
() => {
// Module: crate::iter::blocks
// Provides: {"impl_367"}
// Dependencies: {}
impl < I > ParallelIterator for UniformBlocks < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let callback = BlocksCallback { consumer , sizes : std :: iter :: repeat (self . block_size) , len : self . base . len () , } ; self . base . with_producer (callback) } }
};
}
