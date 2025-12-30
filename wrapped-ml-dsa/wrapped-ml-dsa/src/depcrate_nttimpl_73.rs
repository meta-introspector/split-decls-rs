// Generated macro for impl_73 (impl)
macro_rules! Depcrate_nttimpl_73 {
() => {
// Module: crate::ntt
// Provides: {"impl_73"}
// Dependencies: {}
impl < K : ArraySize > NttInverse for NttVector < K > { type Output = Vector < K > ; fn ntt_inverse (& self) -> Self :: Output { Vector :: new (self . 0 . iter () . map (NttPolynomial :: ntt_inverse) . collect ()) } }
};
}
