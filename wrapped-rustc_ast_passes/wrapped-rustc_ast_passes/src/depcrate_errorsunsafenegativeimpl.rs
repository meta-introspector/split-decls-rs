// Generated macro for UnsafeNegativeImpl (struct)
macro_rules! Depcrate_errorsUnsafeNegativeImpl {
() => {
// Module: crate::errors
// Provides: {"UnsafeNegativeImpl"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_unsafe_negative_impl , code = E0198)] pub (crate) struct UnsafeNegativeImpl { # [primary_span] pub span : Span , # [label (ast_passes_negative)] pub negative : Span , # [label (ast_passes_unsafe)] pub r#unsafe : Span , }
};
}
