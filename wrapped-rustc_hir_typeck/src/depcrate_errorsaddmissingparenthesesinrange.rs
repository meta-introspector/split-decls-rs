// Generated macro for AddMissingParenthesesInRange (struct)
macro_rules! Depcrate_errorsAddMissingParenthesesInRange {
() => {
// Module: crate::errors
// Provides: {"AddMissingParenthesesInRange"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_add_missing_parentheses_in_range , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AddMissingParenthesesInRange { pub func_name : String , # [suggestion_part (code = "(")] pub left : Span , # [suggestion_part (code = ")")] pub right : Span , }
};
}
