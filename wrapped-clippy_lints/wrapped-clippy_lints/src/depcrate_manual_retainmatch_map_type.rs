// Generated macro for match_map_type (function)
macro_rules! Depcrate_manual_retainmatch_map_type {
() => {
// Module: crate::manual_retain
// Provides: {"match_map_type"}
// Dependencies: {}
fn match_map_type (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { let ty = cx . typeck_results () . expr_ty (expr) . peel_refs () ; matches ! (ty . opt_diag_name (cx) , Some (sym :: BTreeMap | sym :: HashMap)) }
};
}
