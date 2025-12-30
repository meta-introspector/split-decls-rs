// Generated macro for is_unit_expression (function)
macro_rules! Depcrate_map_unit_fnis_unit_expression {
() => {
// Module: crate::map_unit_fn
// Provides: {"is_unit_expression"}
// Dependencies: {}
fn is_unit_expression (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { is_unit_type (cx . typeck_results () . expr_ty (expr)) }
};
}
