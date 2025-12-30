// Generated macro for impl_1597 (impl)
macro_rules! Depcrate_sliceimpl_1597 {
() => {
// Module: crate::slice
// Provides: {"impl_1597"}
// Dependencies: {}
impl < T , P : Clone > Clone for Split < '_ , T , P > { fn clone (& self) -> Self { Split { separator : self . separator . clone () , .. * self } } }
};
}
