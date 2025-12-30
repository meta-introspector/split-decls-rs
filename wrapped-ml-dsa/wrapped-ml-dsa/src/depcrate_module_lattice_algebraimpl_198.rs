// Generated macro for impl_198 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_198 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_198"}
// Dependencies: {}
impl < F : Field > Sub < & NttPolynomial < F > > for & NttPolynomial < F > { type Output = NttPolynomial < F > ; fn sub (self , rhs : & NttPolynomial < F >) -> NttPolynomial < F > { NttPolynomial (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (& x , & y) | x - y) . collect () ,) } }
};
}
