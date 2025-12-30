// Generated macro for impl_199 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_199 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_199"}
// Dependencies: {}
impl < F : Field > Mul < & NttPolynomial < F > > for Elem < F > { type Output = NttPolynomial < F > ; fn mul (self , rhs : & NttPolynomial < F >) -> NttPolynomial < F > { NttPolynomial (rhs . 0 . iter () . map (| & x | self * x) . collect ()) } }
};
}
