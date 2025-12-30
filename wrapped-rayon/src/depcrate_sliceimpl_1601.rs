// Generated macro for impl_1601 (impl)
macro_rules! Depcrate_sliceimpl_1601 {
() => {
// Module: crate::slice
// Provides: {"impl_1601"}
// Dependencies: {}
impl < T , P : Clone > Clone for SplitInclusive < '_ , T , P > { fn clone (& self) -> Self { SplitInclusive { separator : self . separator . clone () , .. * self } } }
};
}
