// Generated macro for UnconstrainedGenericParameter (struct)
macro_rules! Depcrate_errorsUnconstrainedGenericParameter {
() => {
// Module: crate::errors
// Provides: {"UnconstrainedGenericParameter"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_unconstrained_generic_parameter)] pub (crate) struct UnconstrainedGenericParameter { # [primary_span] # [label] pub span : Span , pub param_name : Ident , pub param_def_kind : & 'static str , # [note (hir_analysis_const_param_note)] pub const_param_note : bool , # [note (hir_analysis_const_param_note2)] pub const_param_note2 : bool , }
};
}
