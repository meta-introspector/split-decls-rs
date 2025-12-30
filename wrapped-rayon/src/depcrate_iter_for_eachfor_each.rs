// Generated macro for for_each (function)
macro_rules! Depcrate_iter_for_eachfor_each {
() => {
// Module: crate::iter::for_each
// Provides: {"for_each"}
// Dependencies: {}
pub (super) fn for_each < I , F , T > (pi : I , op : & F) where I : ParallelIterator < Item = T > , F : Fn (T) + Sync , T : Send , { let consumer = ForEachConsumer { op } ; pi . drive_unindexed (consumer) }
};
}
