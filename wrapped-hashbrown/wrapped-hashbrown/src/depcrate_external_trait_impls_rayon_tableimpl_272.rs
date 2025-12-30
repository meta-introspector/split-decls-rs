// Generated macro for impl_272 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_272 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_272"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoParIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
};
}
