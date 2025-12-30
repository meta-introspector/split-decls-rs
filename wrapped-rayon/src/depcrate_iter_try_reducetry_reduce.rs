// Generated macro for try_reduce (function)
macro_rules! Depcrate_iter_try_reducetry_reduce {
() => {
// Module: crate::iter::try_reduce
// Provides: {"try_reduce"}
// Dependencies: {}
pub (super) fn try_reduce < PI , R , ID , T > (pi : PI , identity : ID , reduce_op : R) -> T where PI : ParallelIterator < Item = T > , R : Fn (T :: Output , T :: Output) -> T + Sync , ID : Fn () -> T :: Output + Sync , T : Try + Send , { let full = AtomicBool :: new (false) ; let consumer = TryReduceConsumer { identity : & identity , reduce_op : & reduce_op , full : & full , } ; pi . drive_unindexed (consumer) }
};
}
