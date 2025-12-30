// Generated macro for impl_279 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_279 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_279"}
// Dependencies: {}
impl < 'a , T : Send , A : Allocator > IntoParallelIterator for & 'a mut HashTable < T , A > { type Item = & 'a mut T ; type Iter = ParIterMut < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIterMut { inner : unsafe { self . raw . par_iter () } , marker : PhantomData , } } }
};
}
