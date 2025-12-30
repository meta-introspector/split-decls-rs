// Generated macro for PassFnItemToVariadicFunction (struct)
macro_rules! Depcrate_errorsPassFnItemToVariadicFunction {
() => {
// Module: crate::errors
// Provides: {"PassFnItemToVariadicFunction"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_fn_item_to_variadic_function , code = E0617)] # [help] # [note] pub (crate) struct PassFnItemToVariadicFunction { # [primary_span] pub span : Span , # [suggestion (code = " as {replace}" , applicability = "machine-applicable" , style = "verbose")] pub sugg_span : Span , pub replace : String , }
};
}
