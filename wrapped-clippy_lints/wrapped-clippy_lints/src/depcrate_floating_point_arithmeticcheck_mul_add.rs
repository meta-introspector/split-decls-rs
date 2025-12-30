// Generated macro for check_mul_add (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_mul_add {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_mul_add"}
// Dependencies: {}
fn check_mul_add (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Binary (Spanned { node : op @ (BinOpKind :: Add | BinOpKind :: Sub) , .. } , lhs , rhs ,) = & expr . kind { if let Some (parent) = get_parent_expr (cx , expr) && let ExprKind :: MethodCall (PathSegment { ident : method , .. } , receiver , ..) = parent . kind && method . name == sym :: sqrt && detect_hypot (cx , receiver) . is_some () { return ; } let maybe_neg_sugg = | expr | { let sugg = Sugg :: hir (cx , expr , "..") ; if let BinOpKind :: Sub = op { - sugg } else { sugg } } ; let (recv , arg1 , arg2) = if let Some ((inner_lhs , inner_rhs)) = is_float_mul_expr (cx , lhs) && cx . typeck_results () . expr_ty (rhs) . is_floating_point () { (inner_lhs , Sugg :: hir (cx , inner_rhs , "..") , maybe_neg_sugg (rhs)) } else if let Some ((inner_lhs , inner_rhs)) = is_float_mul_expr (cx , rhs) && cx . typeck_results () . expr_ty (lhs) . is_floating_point () { (inner_lhs , maybe_neg_sugg (inner_rhs) , Sugg :: hir (cx , lhs , "..")) } else { return ; } ; if (matches ! (recv . kind , ExprKind :: Path (_)) || matches ! (recv . kind , ExprKind :: Call (_ , _))) && has_ambiguous_literal_in_expr (cx , recv) { return ; } span_lint_and_sugg (cx , SUBOPTIMAL_FLOPS , expr . span , "multiply and add expressions can be calculated more efficiently and accurately" , "consider using" , format ! ("{}.mul_add({arg1}, {arg2})" , prepare_receiver_sugg (cx , recv)) , Applicability :: MachineApplicable ,) ; } }
};
}
