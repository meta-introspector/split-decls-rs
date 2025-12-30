// Generated macro for impl_186 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_186 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_186"}
// Dependencies: {}
impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug , A : Allocator > fmt :: Debug for ParDrain < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
};
}
