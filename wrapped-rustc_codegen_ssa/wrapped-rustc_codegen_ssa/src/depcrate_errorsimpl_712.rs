// Generated macro for impl_712 (impl)
macro_rules! Depcrate_errorsimpl_712 {
() => {
// Module: crate::errors
// Provides: {"impl_712"}
// Dependencies: {}
impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for LinkExeStatusStackBufferOverrun { fn into_diag (self , dcx : rustc_errors :: DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , fluent :: codegen_ssa_link_exe_status_stack_buffer_overrun) ; diag . note (fluent :: codegen_ssa_abort_note) ; diag . note (fluent :: codegen_ssa_event_log_note) ; diag } }
};
}
