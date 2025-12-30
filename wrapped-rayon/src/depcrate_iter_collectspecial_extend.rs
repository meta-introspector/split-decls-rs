// Generated macro for special_extend (function)
macro_rules! Depcrate_iter_collectspecial_extend {
() => {
// Module: crate::iter::collect
// Provides: {"special_extend"}
// Dependencies: {}
# [doc = " Collects the results of the iterator into the specified vector."] # [doc = ""] # [doc = " Technically, this only works for `IndexedParallelIterator`, but we're faking a"] # [doc = " bit of specialization here until Rust can do that natively.  Callers are"] # [doc = " using `opt_len` to find the length before calling this, and only exact"] # [doc = " iterators will return anything but `None` there."] # [doc = ""] # [doc = " Since the type system doesn't understand that contract, we have to allow"] # [doc = " *any* `ParallelIterator` here, and `CollectConsumer` has to also implement"] # [doc = " `UnindexedConsumer`.  That implementation panics `unreachable!` in case"] # [doc = " there's a bug where we actually do try to use this unindexed."] pub (super) fn special_extend < I , T > (pi : I , len : usize , v : & mut Vec < T >) where I : ParallelIterator < Item = T > , T : Send , { collect_with_consumer (v , len , | consumer | pi . drive_unindexed (consumer)) ; }
};
}
