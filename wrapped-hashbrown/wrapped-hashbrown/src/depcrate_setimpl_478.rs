// Generated macro for impl_478 (impl)
macro_rules! Depcrate_setimpl_478 {
() => {
// Module: crate::set
// Provides: {"impl_478"}
// Dependencies: {}
impl < T : Clone , S : Clone , A : Allocator + Clone > Clone for HashSet < T , S , A > { fn clone (& self) -> Self { HashSet { map : self . map . clone () , } } fn clone_from (& mut self , source : & Self) { self . map . clone_from (& source . map) ; } }
};
}
