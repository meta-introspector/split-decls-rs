// Generated macro for impl_473 (impl)
macro_rules! Depcrate_ioimpl_473 {
() => {
// Module: crate::io
// Provides: {"impl_473"}
// Dependencies: {}
impl < T : AsyncWrite , E : From < io :: Error > > Clone for UnsafeWriter < T , E > { fn clone (& self) -> Self { UnsafeWriter (self . 0 . clone () , self . 1 . clone ()) } }
};
}
