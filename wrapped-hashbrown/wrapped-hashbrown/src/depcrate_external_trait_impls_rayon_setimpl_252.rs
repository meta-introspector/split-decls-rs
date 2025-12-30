// Generated macro for impl_252 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_252 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_252"}
// Dependencies: {}
# [doc = " Extend a hash set with copied items from a parallel iterator."] impl < 'a , T , S > ParallelExtend < & 'a T > for HashSet < T , S , Global > where T : 'a + Copy + Eq + Hash + Sync , S : BuildHasher , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend (self , par_iter) ; } }
};
}
