// Generated macro for impl_102 (impl)
macro_rules! Depcrateimpl_102 {
() => {
// Module: crate
// Provides: {"impl_102"}
// Dependencies: {}
impl < T : Clone + Num + Neg < Output = T > > Neg for Complex < T > { type Output = Self ; # [inline] fn neg (self) -> Self :: Output { Self :: Output :: new (- self . re , - self . im) } }
};
}
