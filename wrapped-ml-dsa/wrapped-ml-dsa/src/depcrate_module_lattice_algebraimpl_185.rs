// Generated macro for impl_185 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_185 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_185"}
// Dependencies: {}
impl < F : Field > Mul < & Polynomial < F > > for Elem < F > { type Output = Polynomial < F > ; fn mul (self , rhs : & Polynomial < F >) -> Polynomial < F > { Polynomial (rhs . 0 . iter () . map (| & x | self * x) . collect ()) } }
};
}
