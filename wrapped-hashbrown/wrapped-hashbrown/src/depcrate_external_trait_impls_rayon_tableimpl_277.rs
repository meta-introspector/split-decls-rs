// Generated macro for impl_277 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_277 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_277"}
// Dependencies: {}
impl < T : Send , A : Allocator + Send > IntoParallelIterator for HashTable < T , A > { type Item = T ; type Iter = IntoParIter < T , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { IntoParIter { inner : self . raw . into_par_iter () , } } }
};
}
