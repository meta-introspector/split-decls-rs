// Generated macro for impl_220 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_220 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_220"}
// Dependencies: {}
impl < T , A : Allocator > Drop for RawParDrain < '_ , T , A > { fn drop (& mut self) { unsafe { self . table . as_mut () . clear () ; } } }
};
}
