// Generated macro for impl_205 (impl)
macro_rules! Depcrate_infer_opaque_types_tableimpl_205 {
() => {
// Module: crate::infer::opaque_types::table
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'tcx > Drop for OpaqueTypeStorage < 'tcx > { fn drop (& mut self) { if ! self . is_empty () { ty :: tls :: with (| tcx | tcx . dcx () . delayed_bug (format ! ("{:?}" , self . opaque_types))) ; } } }
};
}
