// Generated macro for impl_16 (impl)
macro_rules! Depcrate_snapshotimpl_16 {
() => {
// Module: crate::snapshot
// Provides: {"impl_16"}
// Dependencies: {}
impl < T : Clone + std :: fmt :: Debug > Clone for FileSnapshot < T > { fn clone (& self) -> Self { Self { value : self . value . clone () , modified : self . modified , } } }
};
}
