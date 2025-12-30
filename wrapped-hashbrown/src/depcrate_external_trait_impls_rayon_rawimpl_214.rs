// Generated macro for impl_214 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_214 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_214"}
// Dependencies: {}
impl < T , A : Allocator > RawIntoParIter < T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub (super) unsafe fn par_iter (& self) -> RawParIter < T > { self . table . par_iter () } }
};
}
