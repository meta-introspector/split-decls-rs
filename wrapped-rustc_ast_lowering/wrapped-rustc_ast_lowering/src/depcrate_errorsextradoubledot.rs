// Generated macro for ExtraDoubleDot (struct)
macro_rules! Depcrate_errorsExtraDoubleDot {
() => {
// Module: crate::errors
// Provides: {"ExtraDoubleDot"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_extra_double_dot)] pub (crate) struct ExtraDoubleDot < 'a > { # [primary_span] # [label] pub span : Span , # [label (ast_lowering_previously_used_here)] pub prev_span : Span , pub ctx : & 'a str , }
};
}
