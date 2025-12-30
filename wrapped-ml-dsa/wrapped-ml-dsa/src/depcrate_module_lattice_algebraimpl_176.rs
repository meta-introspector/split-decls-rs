// Generated macro for impl_176 (impl)
macro_rules! Depcrate_module_lattice_algebraimpl_176 {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"impl_176"}
// Dependencies: {}
impl < F : Field > Neg for Elem < F > { type Output = Elem < F > ; fn neg (self) -> Elem < F > { Elem (F :: small_reduce (F :: Q - self . 0)) } }
};
}
