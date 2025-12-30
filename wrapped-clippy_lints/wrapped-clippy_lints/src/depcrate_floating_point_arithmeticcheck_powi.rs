// Generated macro for check_powi (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_powi {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_powi"}
// Dependencies: {}
fn check_powi (cx : & LateContext < '_ > , expr : & Expr < '_ > , receiver : & Expr < '_ > , args : & [Expr < '_ >]) { if let Some (value) = ConstEvalCtxt :: new (cx) . eval (& args [0]) && value == Int (2) && let Some (parent) = get_parent_expr (cx , expr) { if let Some (grandparent) = get_parent_expr (cx , parent) && let ExprKind :: MethodCall (PathSegment { ident : method , .. } , receiver , ..) = grandparent . kind && method . name == sym :: sqrt && detect_hypot (cx , receiver) . is_some () { return ; } if let ExprKind :: Binary (Spanned { node : op @ (BinOpKind :: Add | BinOpKind :: Sub) , .. } , lhs , rhs ,) = parent . kind { let other_addend = if lhs . hir_id == expr . hir_id { rhs } else { lhs } ; let maybe_neg_sugg = | expr , hir_id | { let sugg = Sugg :: hir (cx , expr , "..") ; if matches ! (op , BinOpKind :: Sub) && hir_id == rhs . hir_id { - sugg } else { sugg } } ; span_lint_and_sugg (cx , SUBOPTIMAL_FLOPS , parent . span , "multiply and add expressions can be calculated more efficiently and accurately" , "consider using" , format ! ("{}.mul_add({}, {})" , Sugg :: hir (cx , receiver , "..") . maybe_paren () , maybe_neg_sugg (receiver , expr . hir_id) , maybe_neg_sugg (other_addend , other_addend . hir_id) ,) , Applicability :: MachineApplicable ,) ; } } }
};
}
