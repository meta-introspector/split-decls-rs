// Generated macro for impl_223 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_223 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_223"}
// Dependencies: {}
impl < T > Drop for ParDrainProducer < T > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { if mem :: needs_drop :: < T > () { for item in & mut self . iter { unsafe { item . drop () ; } } } } }
};
}
