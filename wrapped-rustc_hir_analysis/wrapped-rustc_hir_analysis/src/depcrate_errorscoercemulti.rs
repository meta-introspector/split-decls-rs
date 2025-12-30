// Generated macro for CoerceMulti (struct)
macro_rules! Depcrate_errorsCoerceMulti {
() => {
// Module: crate::errors
// Provides: {"CoerceMulti"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_coerce_multi , code = E0375)] pub (crate) struct CoerceMulti { pub trait_name : & 'static str , # [primary_span] pub span : Span , pub number : usize , # [note] pub fields : MultiSpan , }
};
}
