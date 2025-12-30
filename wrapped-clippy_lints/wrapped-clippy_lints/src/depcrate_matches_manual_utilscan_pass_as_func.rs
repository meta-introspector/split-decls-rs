// Generated macro for can_pass_as_func (function)
macro_rules! Depcrate_matches_manual_utilscan_pass_as_func {
() => {
// Module: crate::matches::manual_utils
// Provides: {"can_pass_as_func"}
// Dependencies: {}
fn can_pass_as_func < 'tcx > (cx : & LateContext < 'tcx > , binding : HirId , expr : & 'tcx Expr < '_ >) -> Option < & 'tcx Expr < 'tcx > > { match expr . kind { ExprKind :: Call (func , [arg]) if arg . res_local_id () == Some (binding) && cx . typeck_results () . expr_adjustments (arg) . is_empty () && ! is_unsafe_fn (cx , cx . typeck_results () . expr_ty (func) . peel_refs ()) => { Some (func) } , _ => None , } }
};
}
