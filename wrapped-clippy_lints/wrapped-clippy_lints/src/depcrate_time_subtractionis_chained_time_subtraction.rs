// Generated macro for is_chained_time_subtraction (function)
macro_rules! Depcrate_time_subtractionis_chained_time_subtraction {
() => {
// Module: crate::time_subtraction
// Provides: {"is_chained_time_subtraction"}
// Dependencies: {}
# [doc = " Returns true if this subtraction is part of a chain like `(a - b) - c`"] fn is_chained_time_subtraction (cx : & LateContext < '_ > , lhs : & Expr < '_ >) -> bool { if let ExprKind :: Binary (op , inner_lhs , inner_rhs) = & lhs . kind && matches ! (op . node , BinOpKind :: Sub) { let typeck = cx . typeck_results () ; let left_ty = typeck . expr_ty (inner_lhs) ; let right_ty = typeck . expr_ty (inner_rhs) ; is_time_type (cx , left_ty) && is_time_type (cx , right_ty) } else { false } }
};
}
