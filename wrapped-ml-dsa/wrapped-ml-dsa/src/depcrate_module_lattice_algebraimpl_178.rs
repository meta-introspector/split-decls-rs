// Generated macro for impl_178 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_178 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_178"}
// Dependencies: {}
impl < F : Field > Sub < Elem < F > > for Elem < F > { type Output = Elem < F > ; fn sub (self , rhs : Elem < F >) -> Elem < F > { Elem (F :: small_reduce (self . 0 + F :: Q - rhs . 0)) } }
};
}
