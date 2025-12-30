// Generated macro for stmt_needs_never_type (function)
macro_rules! Depcrate_returnsstmt_needs_never_type {
() => {
// Module: crate::returns
// Provides: {"stmt_needs_never_type"}
// Dependencies: {}
# [doc = " Checks if a return statement is \"needed\" in the middle of a block, or if it can be removed. This"] # [doc = " is the case when the enclosing block expression is coerced to some other type, which only works"] # [doc = " because of the never-ness of `return` expressions"] fn stmt_needs_never_type (cx : & LateContext < '_ > , stmt_hir_id : HirId) -> bool { cx . tcx . hir_parent_iter (stmt_hir_id) . find_map (| (_ , node) | if let Node :: Expr (expr) = node { Some (expr) } else { None }) . is_some_and (| e | { cx . typeck_results () . expr_adjustments (e) . iter () . any (| adjust | adjust . target != cx . tcx . types . unit && matches ! (adjust . kind , Adjust :: NeverToAny)) }) }
};
}
