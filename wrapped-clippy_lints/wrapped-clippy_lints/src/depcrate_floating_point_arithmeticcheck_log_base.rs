// Generated macro for check_log_base (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_log_base {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_log_base"}
// Dependencies: {}
fn check_log_base (cx : & LateContext < '_ > , expr : & Expr < '_ > , receiver : & Expr < '_ > , args : & [Expr < '_ >]) { if let Some (method) = get_specialized_log_method (cx , & args [0] , expr . span . ctxt ()) { span_lint_and_sugg (cx , SUBOPTIMAL_FLOPS , expr . span , "logarithm for bases 2, 10 and e can be computed more accurately" , "consider using" , format ! ("{}.{method}()" , Sugg :: hir (cx , receiver , "..") . maybe_paren ()) , Applicability :: MachineApplicable ,) ; } }
};
}
