// Generated macro for impl_250 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_250 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_250"}
// Dependencies: {}
# [doc = " Collect values from a parallel iterator into a hashset."] impl < T , S > FromParallelIterator < T > for HashSet < T , S , Global > where T : Eq + Hash + Send , S : BuildHasher + Default , { fn from_par_iter < P > (par_iter : P) -> Self where P : IntoParallelIterator < Item = T > , { let mut set = HashSet :: default () ; set . par_extend (par_iter) ; set } }
};
}
