// Generated macro for AssertMissingComma (struct)
macro_rules! Depcrate_errorsAssertMissingComma {
() => {
// Module: crate::errors
// Provides: {"AssertMissingComma"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_assert_missing_comma)] pub (crate) struct AssertMissingComma { # [primary_span] pub (crate) span : Span , # [suggestion (code = ", " , applicability = "maybe-incorrect" , style = "short")] pub (crate) comma : Span , }
};
}
