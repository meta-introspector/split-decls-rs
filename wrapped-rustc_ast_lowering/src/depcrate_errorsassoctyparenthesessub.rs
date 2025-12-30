// Generated macro for AssocTyParenthesesSub (enum)
macro_rules! Depcrate_errorsAssocTyParenthesesSub {
() => {
// Module: crate::errors
// Provides: {"AssocTyParenthesesSub"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum AssocTyParenthesesSub { # [multipart_suggestion (ast_lowering_remove_parentheses)] Empty { # [suggestion_part (code = "")] parentheses_span : Span , } , # [multipart_suggestion (ast_lowering_use_angle_brackets)] NotEmpty { # [suggestion_part (code = "<")] open_param : Span , # [suggestion_part (code = ">")] close_param : Span , } , }
};
}
