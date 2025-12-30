// Generated macro for is_sub_expr (function)
macro_rules! Depcrate_manual_abs_diffis_sub_expr {
() => {
// Module: crate::manual_abs_diff
// Provides: {"is_sub_expr"}
// Dependencies: {}
# [doc = " Checks if the given expression is a subtraction operation between two expected expressions,"] # [doc = " i.e. if `expr` is `{expected_a} - {expected_b}`."] # [doc = ""] # [doc = " If `expected_ty` is a signed primitive integer, this function will only return `Some` if the"] # [doc = " subtraction expr is wrapped in a cast to the equivalent unsigned int."] fn is_sub_expr (cx : & LateContext < '_ > , expr : & Expr < '_ > , expected_a : & Expr < '_ > , expected_b : & Expr < '_ > , expected_ty : Ty < '_ > ,) -> bool { let expr = peel_blocks (expr) . kind ; if let ty :: Int (ty) = expected_ty . kind () { let unsigned = Ty :: new_uint (cx . tcx , ty . to_unsigned ()) ; return if let ExprKind :: Cast (expr , cast_ty) = expr && cx . typeck_results () . node_type (cast_ty . hir_id) == unsigned { is_sub_expr (cx , expr , expected_a , expected_b , unsigned) } else { false } ; } if let ExprKind :: Binary (op , a , b) = expr && let BinOpKind :: Sub = op . node && eq_expr_value (cx , a , expected_a) && eq_expr_value (cx , b , expected_b) { true } else { false } }
};
}
