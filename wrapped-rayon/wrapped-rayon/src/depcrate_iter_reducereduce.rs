// Generated macro for reduce (function)
macro_rules! Depcrate_iter_reducereduce {
() => {
// Module: crate::iter::reduce
// Provides: {"reduce"}
// Dependencies: {}
pub (super) fn reduce < PI , R , ID , T > (pi : PI , identity : ID , reduce_op : R) -> T where PI : ParallelIterator < Item = T > , R : Fn (T , T) -> T + Sync , ID : Fn () -> T + Sync , T : Send , { let consumer = ReduceConsumer { identity : & identity , reduce_op : & reduce_op , } ; pi . drive_unindexed (consumer) }
};
}
