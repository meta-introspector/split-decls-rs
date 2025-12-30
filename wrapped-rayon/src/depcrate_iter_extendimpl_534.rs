// Generated macro for impl_534 (impl)
macro_rules! Depcrate_iter_extendimpl_534 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_534"}
// Dependencies: {}
# [doc = " Extends a B-tree map with copied items from a parallel iterator."] impl < 'a , K : 'a , V : 'a > ParallelExtend < (& 'a K , & 'a V) > for BTreeMap < K , V > where K : Copy + Ord + Send + Sync , V : Copy + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (& 'a K , & 'a V) > , { extend ! (self , par_iter) ; } }
};
}
