// Generated macro for UnsupportedDelegation (struct)
macro_rules! Depcrate_errorsUnsupportedDelegation {
() => {
// Module: crate::errors
// Provides: {"UnsupportedDelegation"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_not_supported_delegation)] pub (crate) struct UnsupportedDelegation < 'a > { # [primary_span] pub span : Span , pub descr : & 'a str , # [label] pub callee_span : Span , }
};
}
