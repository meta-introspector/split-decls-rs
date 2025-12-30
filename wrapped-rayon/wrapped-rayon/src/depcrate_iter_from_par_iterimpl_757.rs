// Generated macro for impl_757 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_757 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_757"}
// Dependencies: {}
# [doc = " Collects values from a parallel iterator into a hashset."] impl < V , S > FromParallelIterator < V > for HashSet < V , S > where V : Eq + Hash + Send , S : BuildHasher + Default + Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = V > , { collect_extended (par_iter) } }
};
}
