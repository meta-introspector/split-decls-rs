// Generated macro for impl_540 (impl)
macro_rules! Depcrate_iter_extendimpl_540 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_540"}
// Dependencies: {}
# [doc = " Extends a hash set with copied items from a parallel iterator."] impl < 'a , T , S > ParallelExtend < & 'a T > for HashSet < T , S > where T : 'a + Copy + Eq + Hash + Send + Sync , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend_reserved ! (self , par_iter) ; } }
};
}
