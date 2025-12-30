// Generated macro for impl_533 (impl)
macro_rules! Depcrate_iter_extendimpl_533 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_533"}
// Dependencies: {}
# [doc = " Extends a B-tree map with items from a parallel iterator."] impl < K , V > ParallelExtend < (K , V) > for BTreeMap < K , V > where K : Ord + Send , V : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (K , V) > , { extend ! (self , par_iter) ; } }
};
}
