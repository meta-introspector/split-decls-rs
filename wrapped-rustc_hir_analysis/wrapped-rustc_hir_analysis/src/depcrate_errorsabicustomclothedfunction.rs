// Generated macro for AbiCustomClothedFunction (struct)
macro_rules! Depcrate_errorsAbiCustomClothedFunction {
() => {
// Module: crate::errors
// Provides: {"AbiCustomClothedFunction"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_abi_custom_clothed_function)] pub (crate) struct AbiCustomClothedFunction { # [primary_span] pub span : Span , # [suggestion (hir_analysis_suggestion , applicability = "maybe-incorrect" , code = "#[unsafe(naked)]\n" , style = "short")] pub naked_span : Span , }
};
}
