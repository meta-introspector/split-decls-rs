// Generated macro for ExponentialBlocks (struct)
macro_rules! Depcrate_iter_blocksExponentialBlocks {
() => {
// Module: crate::iter::blocks
// Provides: {"ExponentialBlocks"}
// Dependencies: {}
# [doc = " `ExponentialBlocks` is a parallel iterator that consumes itself as a sequence"] # [doc = " of parallel blocks of increasing sizes (exponentially)."] # [doc = ""] # [doc = " This struct is created by the [`by_exponential_blocks()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`by_exponential_blocks()`]: IndexedParallelIterator::by_exponential_blocks()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct ExponentialBlocks < I > { base : I , }
};
}
