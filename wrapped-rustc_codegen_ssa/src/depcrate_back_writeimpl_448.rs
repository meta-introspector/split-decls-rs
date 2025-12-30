// Generated macro for impl_448 (impl)
macro_rules! Depcrate_back_writeimpl_448 {
() => {
// Module: crate::back::write
// Provides: {"impl_448"}
// Dependencies: {}
impl < B : WriteBackendMethods > CodegenContext < B > { pub fn create_dcx (& self) -> DiagCtxt { DiagCtxt :: new (Box :: new (self . diag_emitter . clone ())) } }
};
}
