// Generated macro for impl_183 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_183 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_183"}
// Dependencies: {}
impl < F : Field > Add < & Polynomial < F > > for & Polynomial < F > { type Output = Polynomial < F > ; fn add (self , rhs : & Polynomial < F >) -> Polynomial < F > { Polynomial (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (& x , & y) | x + y) . collect () ,) } }
};
}
