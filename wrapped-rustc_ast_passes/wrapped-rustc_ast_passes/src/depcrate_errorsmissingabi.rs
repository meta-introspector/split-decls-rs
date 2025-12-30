// Generated macro for MissingAbi (struct)
macro_rules! Depcrate_errorsMissingAbi {
() => {
// Module: crate::errors
// Provides: {"MissingAbi"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_extern_without_abi)] # [help] pub (crate) struct MissingAbi { # [primary_span] # [suggestion (code = "extern \"<abi>\"" , applicability = "has-placeholders")] pub span : Span , }
};
}
