// Generated macro for check_hypot (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_hypot {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_hypot"}
// Dependencies: {}
fn check_hypot (cx : & LateContext < '_ > , expr : & Expr < '_ > , receiver : & Expr < '_ >) { if let Some (message) = detect_hypot (cx , receiver) { span_lint_and_sugg (cx , IMPRECISE_FLOPS , expr . span , "hypotenuse can be computed more accurately" , "consider using" , message , Applicability :: MachineApplicable ,) ; } }
};
}
