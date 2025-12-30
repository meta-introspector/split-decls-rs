// Generated macro for CVariadicMustBeUnsafe (struct)
macro_rules! Depcrate_errorsCVariadicMustBeUnsafe {
() => {
// Module: crate::errors
// Provides: {"CVariadicMustBeUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_c_variadic_must_be_unsafe)] pub (crate) struct CVariadicMustBeUnsafe { # [primary_span] pub span : Span , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "unsafe " , style = "verbose")] pub unsafe_span : Span , }
};
}
