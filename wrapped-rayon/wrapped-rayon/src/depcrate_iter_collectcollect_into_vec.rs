// Generated macro for collect_into_vec (function)
macro_rules! Depcrate_iter_collectcollect_into_vec {
() => {
// Module: crate::iter::collect
// Provides: {"collect_into_vec"}
// Dependencies: {}
# [doc = " Collects the results of the exact iterator into the specified vector."] # [doc = ""] # [doc = " This is called by `IndexedParallelIterator::collect_into_vec`."] pub (super) fn collect_into_vec < I , T > (pi : I , v : & mut Vec < T >) where I : IndexedParallelIterator < Item = T > , T : Send , { v . truncate (0) ; let len = pi . len () ; collect_with_consumer (v , len , | consumer | pi . drive (consumer)) ; }
};
}
