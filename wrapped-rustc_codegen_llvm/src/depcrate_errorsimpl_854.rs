// Generated macro for impl_854 (impl)
macro_rules! Depcrate_errorsimpl_854 {
() => {
// Module: crate::errors
// Provides: {"impl_854"}
// Dependencies: {}
impl < G : EmissionGuarantee > Diagnostic < '_ , G > for WithLlvmError < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { use LlvmError :: * ; let msg_with_llvm_err = match & self . 0 { WriteOutput { .. } => fluent :: codegen_llvm_write_output_with_llvm_err , CreateTargetMachine { .. } => fluent :: codegen_llvm_target_machine_with_llvm_err , RunLlvmPasses => fluent :: codegen_llvm_run_passes_with_llvm_err , SerializeModule { .. } => fluent :: codegen_llvm_serialize_module_with_llvm_err , WriteIr { .. } => fluent :: codegen_llvm_write_ir_with_llvm_err , PrepareThinLtoContext => fluent :: codegen_llvm_prepare_thin_lto_context_with_llvm_err , LoadBitcode { .. } => fluent :: codegen_llvm_load_bitcode_with_llvm_err , WriteThinLtoKey { .. } => fluent :: codegen_llvm_write_thinlto_key_with_llvm_err , PrepareThinLtoModule => fluent :: codegen_llvm_prepare_thin_lto_module_with_llvm_err , ParseBitcode => fluent :: codegen_llvm_parse_bitcode_with_llvm_err , PrepareAutoDiff { .. } => fluent :: codegen_llvm_prepare_autodiff_with_llvm_err , } ; self . 0 . into_diag (dcx , level) . with_primary_message (msg_with_llvm_err) . with_arg ("llvm_err" , self . 1) } }
};
}
