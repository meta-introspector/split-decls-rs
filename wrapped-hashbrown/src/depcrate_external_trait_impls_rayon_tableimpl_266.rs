// Generated macro for impl_266 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_266 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_266"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for ParIter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { x . as_ref () }) ; f . debug_list () . entries (iter) . finish () } }
};
}
