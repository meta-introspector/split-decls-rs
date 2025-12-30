// Generated macro for impl_177 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_177 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_177"}
// Dependencies: {}
impl < F : Field > Add < Elem < F > > for Elem < F > { type Output = Elem < F > ; fn add (self , rhs : Elem < F >) -> Elem < F > { Elem (F :: small_reduce (self . 0 + rhs . 0)) } }
};
}
