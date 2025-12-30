// Generated macro for is_reached_through_union (function)
macro_rules! Depcrate_referenceis_reached_through_union {
() => {
// Module: crate::reference
// Provides: {"is_reached_through_union"}
// Dependencies: {}
# [doc = " Checks whether `expr` denotes an object reached through a union"] fn is_reached_through_union (cx : & LateContext < '_ > , mut expr : & Expr < '_ >) -> bool { while let ExprKind :: Field (parent , _) | ExprKind :: Index (parent , _ , _) = expr . kind { if cx . typeck_results () . expr_ty_adjusted (parent) . is_union () { return true ; } expr = parent ; } false }
};
}
