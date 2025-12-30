// Generated macro for impl_191 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_191 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'a , K : Sync , V : Sync , S , A : Allocator > IntoParallelIterator for & 'a HashMap < K , V , S , A > { type Item = (& 'a K , & 'a V) ; type Iter = ParIter < 'a , K , V > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIter { inner : unsafe { self . table . par_iter () } , marker : PhantomData , } } }
};
}
