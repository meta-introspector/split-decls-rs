// Generated macro for ArbitraryExpressionInPattern (struct)
macro_rules! Depcrate_errorsArbitraryExpressionInPattern {
() => {
// Module: crate::errors
// Provides: {"ArbitraryExpressionInPattern"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_arbitrary_expression_in_pattern)] pub (crate) struct ArbitraryExpressionInPattern { # [primary_span] pub span : Span , # [note (ast_lowering_pattern_from_macro_note)] pub pattern_from_macro_note : bool , }
};
}
