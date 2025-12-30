// Generated macro for impl_104 (impl)
macro_rules! Depcrateimpl_104 {
() => {
// Module: crate
// Provides: {"impl_104"}
// Dependencies: {}
impl < T : Clone + Num + Neg < Output = T > > Inv for Complex < T > { type Output = Self ; # [inline] fn inv (self) -> Self :: Output { Complex :: inv (& self) } }
};
}
