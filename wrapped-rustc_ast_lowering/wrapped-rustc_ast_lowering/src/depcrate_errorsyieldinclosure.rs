// Generated macro for YieldInClosure (struct)
macro_rules! Depcrate_errorsYieldInClosure {
() => {
// Module: crate::errors
// Provides: {"YieldInClosure"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_yield_in_closure)] pub (crate) struct YieldInClosure { # [primary_span] pub span : Span , # [suggestion (code = "#[coroutine] " , applicability = "maybe-incorrect" , style = "verbose")] pub suggestion : Option < Span > , }
};
}
