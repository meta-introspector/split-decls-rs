// Generated macro for impl_363 (impl)
macro_rules! Depcrate_iter_blocksimpl_363 {
() => {
// Module: crate::iter::blocks
// Provides: {"impl_363"}
// Dependencies: {}
impl < I > ParallelIterator for ExponentialBlocks < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let first = crate :: current_num_threads () ; let callback = BlocksCallback { consumer , sizes : std :: iter :: successors (Some (first) , exponential_size) , len : self . base . len () , } ; self . base . with_producer (callback) } }
};
}
