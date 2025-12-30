// Generated macro for impl_207 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_207 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_207"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Mul < & NttVector < F , K > > for & NttVector < F , K > where for < 'a > & 'a NttPolynomial < F > : Mul < & 'a NttPolynomial < F > , Output = NttPolynomial < F > > , { type Output = NttPolynomial < F > ; fn mul (self , rhs : & NttVector < F , K >) -> NttPolynomial < F > { self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (x , y) | x * y) . fold (NttPolynomial :: default () , | x , y | & x + & y) } }
};
}
