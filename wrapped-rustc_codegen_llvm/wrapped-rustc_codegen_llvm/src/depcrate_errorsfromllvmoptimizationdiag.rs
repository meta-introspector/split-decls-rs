// Generated macro for FromLlvmOptimizationDiag (struct)
macro_rules! Depcrate_errorsFromLlvmOptimizationDiag {
() => {
// Module: crate::errors
// Provides: {"FromLlvmOptimizationDiag"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_llvm_from_llvm_optimization_diag)] pub (crate) struct FromLlvmOptimizationDiag < 'a > { pub filename : & 'a str , pub line : std :: ffi :: c_uint , pub column : std :: ffi :: c_uint , pub pass_name : & 'a str , pub kind : & 'a str , pub message : & 'a str , }
};
}
