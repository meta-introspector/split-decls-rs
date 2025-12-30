// Generated macro for impl_103 (impl)
macro_rules! Depcrateimpl_103 {
() => {
// Module: crate
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a , T : Clone + Num + Neg < Output = T > > Neg for & 'a Complex < T > { type Output = Complex < T > ; # [inline] fn neg (self) -> Self :: Output { - self . clone () } }
};
}
