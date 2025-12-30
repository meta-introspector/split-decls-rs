// Generated macro for Diagnostic (enum)
macro_rules! Depcrate_llvm_diagnosticDiagnostic {
() => {
// Module: crate::llvm::diagnostic
// Provides: {"Diagnostic"}
// Dependencies: {}
pub (crate) enum Diagnostic < 'll > { Optimization (OptimizationDiagnostic < 'll >) , InlineAsm (InlineAsmDiagnostic) , PGO (& 'll DiagnosticInfo) , Linker (& 'll DiagnosticInfo) , Unsupported (& 'll DiagnosticInfo) , # [doc = " LLVM has other types that we do not wrap here."] # [expect (dead_code)] UnknownDiagnostic (& 'll DiagnosticInfo) , }
};
}
