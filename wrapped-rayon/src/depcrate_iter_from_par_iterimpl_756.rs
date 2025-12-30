// Generated macro for impl_756 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_756 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_756"}
// Dependencies: {}
# [doc = " Collects (key, value) pairs from a parallel iterator into a"] # [doc = " btreemap. If multiple pairs correspond to the same key, then the"] # [doc = " ones produced earlier in the parallel iterator will be"] # [doc = " overwritten, just as with a sequential iterator."] impl < K , V > FromParallelIterator < (K , V) > for BTreeMap < K , V > where K : Ord + Send , V : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = (K , V) > , { collect_extended (par_iter) } }
};
}
