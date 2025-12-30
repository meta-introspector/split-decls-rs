// Generated macro for detect_hypot (function)
macro_rules! Depcrate_floating_point_arithmeticdetect_hypot {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"detect_hypot"}
// Dependencies: {}
fn detect_hypot (cx : & LateContext < '_ > , receiver : & Expr < '_ >) -> Option < String > { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Add , .. } , add_lhs , add_rhs ,) = receiver . kind { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Mul , .. } , lmul_lhs , lmul_rhs ,) = add_lhs . kind && let ExprKind :: Binary (Spanned { node : BinOpKind :: Mul , .. } , rmul_lhs , rmul_rhs ,) = add_rhs . kind && eq_expr_value (cx , lmul_lhs , lmul_rhs) && eq_expr_value (cx , rmul_lhs , rmul_rhs) { return Some (format ! ("{}.hypot({})" , Sugg :: hir (cx , lmul_lhs , "..") . maybe_paren () , Sugg :: hir (cx , rmul_lhs , ".."))) ; } if let ExprKind :: MethodCall (PathSegment { ident : lmethod , .. } , largs_0 , [largs_1 , ..] , _) = & add_lhs . kind && let ExprKind :: MethodCall (PathSegment { ident : rmethod , .. } , rargs_0 , [rargs_1 , ..] , _) = & add_rhs . kind && lmethod . name == sym :: powi && rmethod . name == sym :: powi && let ecx = ConstEvalCtxt :: new (cx) && let Some (lvalue) = ecx . eval (largs_1) && let Some (rvalue) = ecx . eval (rargs_1) && Int (2) == lvalue && Int (2) == rvalue { return Some (format ! ("{}.hypot({})" , Sugg :: hir (cx , largs_0 , "..") . maybe_paren () , Sugg :: hir (cx , rargs_0 , ".."))) ; } } None }
};
}
