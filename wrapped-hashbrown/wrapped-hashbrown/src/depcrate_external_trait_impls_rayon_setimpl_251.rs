// Generated macro for impl_251 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_251 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_251"}
// Dependencies: {}
# [doc = " Extend a hash set with items from a parallel iterator."] impl < T , S > ParallelExtend < T > for HashSet < T , S , Global > where T : Eq + Hash + Send , S : BuildHasher , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend (self , par_iter) ; } }
};
}
