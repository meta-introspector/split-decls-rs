// Generated macro for check (function)
macro_rules! Depcrate_methods_iter_nthcheck {
() => {
// Module: crate::methods::iter_nth
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & hir :: Expr < '_ > , iter_recv : & 'tcx hir :: Expr < 'tcx > , iter_method : Symbol , iter_span : Span , nth_span : Span ,) -> bool { let caller_type = match cx . typeck_results () . expr_ty (iter_recv) . peel_refs () . opt_diag_name (cx) { Some (sym :: Vec) => "`Vec`" , Some (sym :: VecDeque) => "`VecDeque`" , _ if cx . typeck_results () . expr_ty_adjusted (iter_recv) . peel_refs () . is_slice () => "slice" , _ => return false , } ; span_lint_and_then (cx , ITER_NTH , expr . span , format ! ("called `.{iter_method}().nth()` on a {caller_type}") , | diag | { let get_method = if iter_method == sym :: iter_mut { "get_mut" } else { "get" } ; diag . span_suggestion_verbose (iter_span . to (nth_span) , format ! ("`{get_method}` is equivalent but more concise") , get_method , Applicability :: MachineApplicable ,) ; } ,) ; true }
};
}
