// Generated macro for try_reduce_with (function)
macro_rules! Depcrate_iter_try_reduce_withtry_reduce_with {
() => {
// Module: crate::iter::try_reduce_with
// Provides: {"try_reduce_with"}
// Dependencies: {}
pub (super) fn try_reduce_with < PI , R , T > (pi : PI , reduce_op : R) -> Option < T > where PI : ParallelIterator < Item = T > , R : Fn (T :: Output , T :: Output) -> T + Sync , T : Try + Send , { let full = AtomicBool :: new (false) ; let consumer = TryReduceWithConsumer { reduce_op : & reduce_op , full : & full , } ; pi . drive_unindexed (consumer) }
};
}
