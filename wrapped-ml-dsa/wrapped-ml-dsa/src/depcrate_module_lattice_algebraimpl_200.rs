// Generated macro for impl_200 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_200 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_200"}
// Dependencies: {}
impl < F : Field > Neg for & NttPolynomial < F > { type Output = NttPolynomial < F > ; fn neg (self) -> NttPolynomial < F > { NttPolynomial (self . 0 . iter () . map (| & x | - x) . collect ()) } }
};
}
