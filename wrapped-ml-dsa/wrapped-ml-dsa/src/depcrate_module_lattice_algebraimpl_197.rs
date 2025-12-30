// Generated macro for impl_197 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_197 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_197"}
// Dependencies: {}
impl < F : Field > Add < & NttPolynomial < F > > for & NttPolynomial < F > { type Output = NttPolynomial < F > ; fn add (self , rhs : & NttPolynomial < F >) -> NttPolynomial < F > { NttPolynomial (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (& x , & y) | x + y) . collect () ,) } }
};
}
