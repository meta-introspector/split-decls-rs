// Generated macro for impl_70 (impl)
macro_rules! Depcrate_nttimpl_70 {
() => {
// Module: crate::ntt
// Provides: {"impl_70"}
// Dependencies: {}
impl < K : ArraySize > Ntt for Vector < K > { type Output = NttVector < K > ; fn ntt (& self) -> Self :: Output { NttVector :: new (self . 0 . iter () . map (Polynomial :: ntt) . collect ()) } }
};
}
