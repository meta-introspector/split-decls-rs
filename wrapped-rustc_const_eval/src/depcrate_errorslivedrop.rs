// Generated macro for LiveDrop (struct)
macro_rules! Depcrate_errorsLiveDrop {
() => {
// Module: crate::errors
// Provides: {"LiveDrop"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_live_drop , code = E0493)] pub struct LiveDrop < 'tcx > { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , pub dropped_ty : Ty < 'tcx > , # [label (const_eval_dropped_at_label)] pub dropped_at : Span , }
};
}
