// Generated macro for UseIsEmpty (struct)
macro_rules! Depcrate_errorsUseIsEmpty {
() => {
// Module: crate::errors
// Provides: {"UseIsEmpty"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_use_is_empty , applicability = "maybe-incorrect" , style = "verbose")] pub (crate) struct UseIsEmpty < 'tcx > { # [suggestion_part (code = "!")] pub lo : Span , # [suggestion_part (code = ".is_empty()")] pub hi : Span , pub expr_ty : Ty < 'tcx > , }
};
}
