// Generated macro for impl_190 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_190 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_190"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Add < & Vector < F , K > > for & Vector < F , K > { type Output = Vector < F , K > ; fn add (self , rhs : & Vector < F , K >) -> Vector < F , K > { Vector (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (x , y) | x + y) . collect () ,) } }
};
}
