// Generated macro for UseConstGenericArg (struct)
macro_rules! Depcrate_errorsUseConstGenericArg {
() => {
// Module: crate::errors
// Provides: {"UseConstGenericArg"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (ast_lowering_invalid_legacy_const_generic_arg_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UseConstGenericArg { # [suggestion_part (code = "::<{const_args}>")] pub end_of_fn : Span , pub const_args : String , pub other_args : String , # [suggestion_part (code = "{other_args}")] pub call_args : Span , }
};
}
