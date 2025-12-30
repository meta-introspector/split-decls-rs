// Generated macro for impl_206 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_206 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_206"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Mul < & NttVector < F , K > > for & NttPolynomial < F > where for < 'a > & 'a NttPolynomial < F > : Mul < & 'a NttPolynomial < F > , Output = NttPolynomial < F > > , { type Output = NttVector < F , K > ; fn mul (self , rhs : & NttVector < F , K >) -> NttVector < F , K > { NttVector (rhs . 0 . iter () . map (| x | self * x) . collect ()) } }
};
}
