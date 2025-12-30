// Generated macro for impl_247 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_247 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_247"}
// Dependencies: {}
impl < T , S , A > HashSet < T , S , A > where T : Eq + Hash + Send , A : Allocator + Send , { # [doc = " Consumes (potentially in parallel) all values in an arbitrary order,"] # [doc = " while preserving the set's allocated memory for reuse."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_drain (& mut self) -> ParDrain < '_ , T , A > { ParDrain { inner : self . map . par_drain () , } } }
};
}
