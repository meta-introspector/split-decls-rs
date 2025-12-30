// Generated macro for impl_537 (impl)
macro_rules! Depcrate_iter_extendimpl_537 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_537"}
// Dependencies: {}
# [doc = " Extends a hash map with items from a parallel iterator."] impl < K , V , S > ParallelExtend < (K , V) > for HashMap < K , V , S > where K : Eq + Hash + Send , V : Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (K , V) > , { extend_reserved ! (self , par_iter) ; } }
};
}
