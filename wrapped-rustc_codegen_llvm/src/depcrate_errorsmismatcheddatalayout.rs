// Generated macro for MismatchedDataLayout (struct)
macro_rules! Depcrate_errorsMismatchedDataLayout {
() => {
// Module: crate::errors
// Provides: {"MismatchedDataLayout"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_llvm_mismatch_data_layout)] pub (crate) struct MismatchedDataLayout < 'a > { pub rustc_target : & 'a str , pub rustc_layout : & 'a str , pub llvm_target : & 'a str , pub llvm_layout : & 'a str , }
};
}
