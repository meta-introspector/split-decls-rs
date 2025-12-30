// Generated macro for impl_179 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_179 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_179"}
// Dependencies: {}
impl < F : Field > Mul < Elem < F > > for Elem < F > { type Output = Elem < F > ; fn mul (self , rhs : Elem < F >) -> Elem < F > { let lhs : F :: Long = self . 0 . into () ; let rhs : F :: Long = rhs . 0 . into () ; let prod = lhs * rhs ; Elem (F :: barrett_reduce (prod)) } }
};
}
