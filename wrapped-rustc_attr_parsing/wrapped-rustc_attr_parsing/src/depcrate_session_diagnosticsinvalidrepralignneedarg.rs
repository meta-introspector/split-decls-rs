// Generated macro for InvalidReprAlignNeedArg (struct)
macro_rules! Depcrate_session_diagnosticsInvalidReprAlignNeedArg {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidReprAlignNeedArg"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_invalid_repr_align_need_arg , code = E0589)] pub (crate) struct InvalidReprAlignNeedArg { # [primary_span] # [suggestion (code = "align(...)" , applicability = "has-placeholders")] pub span : Span , }
};
}
