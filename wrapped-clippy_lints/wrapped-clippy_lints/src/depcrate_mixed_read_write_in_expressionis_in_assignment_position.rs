// Generated macro for is_in_assignment_position (function)
macro_rules! Depcrate_mixed_read_write_in_expressionis_in_assignment_position {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"is_in_assignment_position"}
// Dependencies: {}
# [doc = " Returns `true` if `expr` is the LHS of an assignment, like `expr = ...`."] fn is_in_assignment_position (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (parent) = get_parent_expr (cx , expr) && let ExprKind :: Assign (lhs , ..) = parent . kind { return lhs . hir_id == expr . hir_id ; } false }
};
}
