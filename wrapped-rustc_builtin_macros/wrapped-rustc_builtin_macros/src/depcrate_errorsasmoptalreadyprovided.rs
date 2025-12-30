// Generated macro for AsmOptAlreadyprovided (struct)
macro_rules! Depcrate_errorsAsmOptAlreadyprovided {
() => {
// Module: crate::errors
// Provides: {"AsmOptAlreadyprovided"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_asm_opt_already_provided)] pub (crate) struct AsmOptAlreadyprovided { # [primary_span] # [label] pub (crate) span : Span , pub (crate) symbol : Symbol , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) span_with_comma : Span , }
};
}
