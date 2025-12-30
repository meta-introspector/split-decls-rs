// Generated macro for impl_276 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_276 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_276"}
// Dependencies: {}
impl < T : Send , A : Allocator > HashTable < T , A > { # [doc = " Consumes (potentially in parallel) all values in an arbitrary order,"] # [doc = " while preserving the map's allocated memory for reuse."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_drain (& mut self) -> ParDrain < '_ , T , A > { ParDrain { inner : self . raw . par_drain () , } } }
};
}
