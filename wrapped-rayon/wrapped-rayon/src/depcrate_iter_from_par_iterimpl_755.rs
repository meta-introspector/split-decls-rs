// Generated macro for impl_755 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_755 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_755"}
// Dependencies: {}
# [doc = " Collects (key, value) pairs from a parallel iterator into a"] # [doc = " hashmap. If multiple pairs correspond to the same key, then the"] # [doc = " ones produced earlier in the parallel iterator will be"] # [doc = " overwritten, just as with a sequential iterator."] impl < K , V , S > FromParallelIterator < (K , V) > for HashMap < K , V , S > where K : Eq + Hash + Send , V : Send , S : BuildHasher + Default + Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = (K , V) > , { collect_extended (par_iter) } }
};
}
