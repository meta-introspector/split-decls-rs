// Generated macro for CantDereference (struct)
macro_rules! Depcrate_errorsCantDereference {
() => {
// Module: crate::errors
// Provides: {"CantDereference"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_cant_dereference , code = E0614)] pub (crate) struct CantDereference < 'tcx > { # [primary_span] # [label (hir_typeck_cant_dereference_label)] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , }
};
}
