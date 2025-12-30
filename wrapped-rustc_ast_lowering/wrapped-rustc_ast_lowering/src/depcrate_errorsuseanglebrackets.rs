// Generated macro for UseAngleBrackets (struct)
macro_rules! Depcrate_errorsUseAngleBrackets {
() => {
// Module: crate::errors
// Provides: {"UseAngleBrackets"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (ast_lowering_use_angle_brackets , applicability = "maybe-incorrect")] pub (crate) struct UseAngleBrackets { # [suggestion_part (code = "<")] pub open_param : Span , # [suggestion_part (code = ">")] pub close_param : Span , }
};
}
