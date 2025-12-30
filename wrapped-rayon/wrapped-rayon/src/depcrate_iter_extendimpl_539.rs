// Generated macro for impl_539 (impl)
macro_rules! Depcrate_iter_extendimpl_539 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_539"}
// Dependencies: {}
# [doc = " Extends a hash set with items from a parallel iterator."] impl < T , S > ParallelExtend < T > for HashSet < T , S > where T : Eq + Hash + Send , S : BuildHasher + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend_reserved ! (self , par_iter) ; } }
};
}
