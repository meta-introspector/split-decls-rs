// Generated macro for BadReturnTypeNotation (enum)
macro_rules! Depcrate_errorsBadReturnTypeNotation {
() => {
// Module: crate::errors
// Provides: {"BadReturnTypeNotation"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum BadReturnTypeNotation { # [diag (ast_lowering_bad_return_type_notation_inputs)] Inputs { # [primary_span] # [suggestion (code = "(..)" , applicability = "machine-applicable" , style = "verbose")] span : Span , } , # [diag (ast_lowering_bad_return_type_notation_output)] Output { # [primary_span] span : Span , # [subdiagnostic] suggestion : RTNSuggestion , } , # [diag (ast_lowering_bad_return_type_notation_needs_dots)] NeedsDots { # [primary_span] # [suggestion (code = "(..)" , applicability = "machine-applicable" , style = "verbose")] span : Span , } , # [diag (ast_lowering_bad_return_type_notation_position)] Position { # [primary_span] span : Span , } , }
};
}
