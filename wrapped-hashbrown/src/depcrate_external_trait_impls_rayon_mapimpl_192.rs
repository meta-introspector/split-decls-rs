// Generated macro for impl_192 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_192 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_192"}
// Dependencies: {}
impl < 'a , K : Sync , V : Send , S , A : Allocator > IntoParallelIterator for & 'a mut HashMap < K , V , S , A > { type Item = (& 'a K , & 'a mut V) ; type Iter = ParIterMut < 'a , K , V > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIterMut { inner : unsafe { self . table . par_iter () } , marker : PhantomData , } } }
};
}
