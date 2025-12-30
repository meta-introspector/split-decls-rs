// Generated macro for impl_42 (impl)
macro_rules! Depcrate_implsimpl_42 {
() => {
// Module: crate::impls
// Provides: {"impl_42"}
// Dependencies: {}
impl < T : Clone , N : ArrayLength > Clone for GenericArray < T , N > { # [inline] fn clone (& self) -> GenericArray < T , N > { self . map (Clone :: clone) } }
};
}
