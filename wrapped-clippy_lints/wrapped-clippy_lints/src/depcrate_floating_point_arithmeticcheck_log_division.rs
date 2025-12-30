// Generated macro for check_log_division (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_log_division {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_log_division"}
// Dependencies: {}
fn check_log_division (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Div , .. } , lhs , rhs ,) = & expr . kind && are_same_base_logs (cx , lhs , rhs) && let ExprKind :: MethodCall (_ , largs_self , ..) = & lhs . kind && let ExprKind :: MethodCall (_ , rargs_self , ..) = & rhs . kind { span_lint_and_sugg (cx , SUBOPTIMAL_FLOPS , expr . span , "log base can be expressed more clearly" , "consider using" , format ! ("{}.log({})" , Sugg :: hir (cx , largs_self , "..") . maybe_paren () , Sugg :: hir (cx , rargs_self , "..") ,) , Applicability :: MachineApplicable ,) ; } }
};
}
