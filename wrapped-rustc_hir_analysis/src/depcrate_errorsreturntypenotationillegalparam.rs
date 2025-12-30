// Generated macro for ReturnTypeNotationIllegalParam (enum)
macro_rules! Depcrate_errorsReturnTypeNotationIllegalParam {
() => {
// Module: crate::errors
// Provides: {"ReturnTypeNotationIllegalParam"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum ReturnTypeNotationIllegalParam { # [diag (hir_analysis_return_type_notation_illegal_param_type)] Type { # [primary_span] span : Span , # [label] param_span : Span , } , # [diag (hir_analysis_return_type_notation_illegal_param_const)] Const { # [primary_span] span : Span , # [label] param_span : Span , } , }
};
}
