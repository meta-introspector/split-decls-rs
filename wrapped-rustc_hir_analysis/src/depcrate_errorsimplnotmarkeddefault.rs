// Generated macro for ImplNotMarkedDefault (enum)
macro_rules! Depcrate_errorsImplNotMarkedDefault {
() => {
// Module: crate::errors
// Provides: {"ImplNotMarkedDefault"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum ImplNotMarkedDefault { # [diag (hir_analysis_impl_not_marked_default , code = E0520)] # [note] Ok { # [primary_span] # [label] span : Span , # [label (hir_analysis_ok_label)] ok_label : Span , ident : Ident , } , # [diag (hir_analysis_impl_not_marked_default_err , code = E0520)] # [note] Err { # [primary_span] span : Span , cname : Symbol , ident : Ident , } , }
};
}
