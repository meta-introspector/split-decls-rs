// Generated macro for check (function)
macro_rules! Depcrate_methods_iter_out_of_boundscheck {
() => {
// Module: crate::methods::iter_out_of_bounds
// Provides: {"check"}
// Dependencies: {}
fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , recv : & 'tcx Expr < 'tcx > , arg : & 'tcx Expr < 'tcx > , message : & 'static str , note : & 'static str ,) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let Some (len) = get_iterator_length (cx , recv) && let Some (skipped) = expr_as_u128 (cx , arg) && skipped > len { span_lint_and_note (cx , ITER_OUT_OF_BOUNDS , expr . span , message , None , note) ; } }
};
}
