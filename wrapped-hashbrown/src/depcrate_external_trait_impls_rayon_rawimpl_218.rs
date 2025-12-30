// Generated macro for impl_218 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_218 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_218"}
// Dependencies: {}
impl < T , A : Allocator > RawParDrain < '_ , T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub (super) unsafe fn par_iter (& self) -> RawParIter < T > { self . table . as_ref () . par_iter () } }
};
}
