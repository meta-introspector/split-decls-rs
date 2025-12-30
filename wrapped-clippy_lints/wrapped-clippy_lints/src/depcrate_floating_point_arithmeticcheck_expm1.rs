// Generated macro for check_expm1 (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_expm1 {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_expm1"}
// Dependencies: {}
fn check_expm1 (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Sub , .. } , lhs , rhs ,) = expr . kind && let ExprKind :: MethodCall (path , self_arg , [] , _) = & lhs . kind && path . ident . name == sym :: exp && cx . typeck_results () . expr_ty (lhs) . is_floating_point () && let Some (value) = ConstEvalCtxt :: new (cx) . eval (rhs) && (F32 (1.0) == value || F64 (1.0) == value) && cx . typeck_results () . expr_ty (self_arg) . is_floating_point () { span_lint_and_sugg (cx , IMPRECISE_FLOPS , expr . span , "(e.pow(x) - 1) can be computed more accurately" , "consider using" , format ! ("{}.exp_m1()" , Sugg :: hir (cx , self_arg , "..") . maybe_paren ()) , Applicability :: MachineApplicable ,) ; } }
};
}
