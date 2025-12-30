// Generated macro for impl_186 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_186 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_186"}
// Dependencies: {}
impl < F : Field > Neg for & Polynomial < F > { type Output = Polynomial < F > ; fn neg (self) -> Polynomial < F > { Polynomial (self . 0 . iter () . map (| & x | - x) . collect ()) } }
};
}
