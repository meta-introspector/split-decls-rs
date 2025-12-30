// Generated macro for impl_758 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_758 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_758"}
// Dependencies: {}
# [doc = " Collects values from a parallel iterator into a btreeset."] impl < V > FromParallelIterator < V > for BTreeSet < V > where V : Send + Ord , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = V > , { collect_extended (par_iter) } }
};
}
