// Generated macro for impl_183 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_183 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_183"}
// Dependencies: {}
impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug , A : Allocator > fmt :: Debug for IntoParIter < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
};
}
