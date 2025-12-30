// Generated macro for impl_105 (impl)
macro_rules! Depcrateimpl_105 {
() => {
// Module: crate
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a , T : Clone + Num + Neg < Output = T > > Inv for & 'a Complex < T > { type Output = Complex < T > ; # [inline] fn inv (self) -> Self :: Output { Complex :: inv (self) } }
};
}
