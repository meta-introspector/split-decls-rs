// Generated macro for impl_538 (impl)
macro_rules! Depcrate_iter_extendimpl_538 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_538"}
// Dependencies: {}
# [doc = " Extends a hash map with copied items from a parallel iterator."] impl < 'a , K : 'a , V : 'a , S > ParallelExtend < (& 'a K , & 'a V) > for HashMap < K , V , S > where K : Copy + Eq + Hash + Send + Sync , V : Copy + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = (& 'a K , & 'a V) > , { extend_reserved ! (self , par_iter) ; } }
};
}
