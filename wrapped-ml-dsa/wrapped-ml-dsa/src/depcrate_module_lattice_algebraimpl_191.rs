// Generated macro for impl_191 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_191 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_191"}
// Dependencies: {}
impl < F : Field , K : ArraySize > Sub < & Vector < F , K > > for & Vector < F , K > { type Output = Vector < F , K > ; fn sub (self , rhs : & Vector < F , K >) -> Vector < F , K > { Vector (self . 0 . iter () . zip (rhs . 0 . iter ()) . map (| (x , y) | x - y) . collect () ,) } }
};
}
