// Generated macro for InvalidSafetyOnExtern (struct)
macro_rules! Depcrate_errorsInvalidSafetyOnExtern {
() => {
// Module: crate::errors
// Provides: {"InvalidSafetyOnExtern"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_extern_invalid_safety)] pub (crate) struct InvalidSafetyOnExtern { # [primary_span] pub item_span : Span , # [suggestion (code = "unsafe " , applicability = "machine-applicable" , style = "verbose")] pub block : Option < Span > , }
};
}
