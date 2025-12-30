// Generated macro for impl_193 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_193 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_193"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Neg for & Vector < F , K > { type Output = Vector < F , K > ; fn neg (self) -> Vector < F , K > { Vector (self . 0 . iter () . map (| x | - x) . collect ()) } }
};
}
