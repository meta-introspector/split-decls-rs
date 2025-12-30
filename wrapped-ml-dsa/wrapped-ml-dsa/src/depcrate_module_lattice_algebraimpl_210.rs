// Generated macro for impl_210 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_210 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_210"}
// Dependencies: {}
impl < F : Field , K : ArraySize , L : ArraySize > Mul < & NttVector < F , L > > for & NttMatrix < F , K , L > where for < 'a > & 'a NttPolynomial < F > : Mul < & 'a NttPolynomial < F > , Output = NttPolynomial < F > > , { type Output = NttVector < F , K > ; fn mul (self , rhs : & NttVector < F , L >) -> NttVector < F , K > { NttVector (self . 0 . iter () . map (| x | x * rhs) . collect ()) } }
};
}
