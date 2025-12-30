// Generated macro for impl_170 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_170 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_170"}
// Dependencies: {}
impl < K : fmt :: Debug + Eq + Hash , V > fmt :: Debug for ParKeys < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let iter = unsafe { self . inner . iter () } . map (| x | unsafe { & x . as_ref () . 0 }) ; f . debug_list () . entries (iter) . finish () } }
};
}
