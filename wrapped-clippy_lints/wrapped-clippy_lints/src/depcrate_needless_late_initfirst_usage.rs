// Generated macro for first_usage (function)
macro_rules! Depcrate_needless_late_initfirst_usage {
() => {
// Module: crate::needless_late_init
// Provides: {"first_usage"}
// Dependencies: {}
fn first_usage < 'tcx > (cx : & LateContext < 'tcx > , binding_id : HirId , local_stmt_id : HirId , block : & 'tcx Block < 'tcx > ,) -> Option < Usage < 'tcx > > { let significant_drop = needs_ordered_drop (cx , cx . typeck_results () . node_type (binding_id)) ; block . stmts . iter () . skip_while (| stmt | stmt . hir_id != local_stmt_id) . skip (1) . take_while (| stmt | ! significant_drop || ! stmt_needs_ordered_drop (cx , stmt)) . find (| & stmt | is_local_used (cx , stmt , binding_id)) . and_then (| stmt | match stmt . kind { StmtKind :: Expr (expr) => Some (Usage { stmt , expr , needs_semi : true , }) , StmtKind :: Semi (expr) => Some (Usage { stmt , expr , needs_semi : false , }) , _ => None , }) }
};
}
