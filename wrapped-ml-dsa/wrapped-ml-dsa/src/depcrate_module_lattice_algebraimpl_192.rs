// Generated macro for impl_192 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_192 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_192"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Mul < & Vector < F , K > > for Elem < F > { type Output = Vector < F , K > ; fn mul (self , rhs : & Vector < F , K >) -> Vector < F , K > { Vector (rhs . 0 . iter () . map (| x | self * x) . collect ()) } }
};
}
