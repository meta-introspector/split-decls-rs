// Generated macro for impl_184 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_184 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_184"}
// Dependencies: {}
impl < F : Field > Sub < & Polynomial < F > > for & Polynomial < F > { type Output = Polynomial < F > ; fn sub (self , rhs : & Polynomial < F >) -> Polynomial < F > { Polynomial (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (& x , & y) | x - y) . collect () ,) } }
};
}
