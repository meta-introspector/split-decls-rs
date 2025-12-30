// Generated macro for impl_278 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_278 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'a , T : Sync , A : Allocator > IntoParallelIterator for & 'a HashTable < T , A > { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIter { inner : unsafe { self . raw . par_iter () } , marker : PhantomData , } } }
};
}
