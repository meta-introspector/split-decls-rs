// Generated macro for impl_275 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_275 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_275"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for ParDrain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
};
}
