// Generated macro for InvalidAbi (struct)
macro_rules! Depcrate_errorsInvalidAbi {
() => {
// Module: crate::errors
// Provides: {"InvalidAbi"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_invalid_abi , code = E0703)] # [note] pub (crate) struct InvalidAbi { # [primary_span] # [label] pub span : Span , pub abi : Symbol , pub command : String , # [subdiagnostic] pub suggestion : Option < InvalidAbiSuggestion > , }
};
}
