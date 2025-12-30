// Generated macro for impl_11133 (impl)
macro_rules! Depcrate_unused_roundingimpl_11133 {
() => {
// Module: crate::unused_rounding
// Provides: {"impl_11133"}
// Dependencies: {}
impl EarlyLintPass for UnusedRounding { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let Some ((method_name , float)) = is_useless_rounding (cx , expr) { span_lint_and_sugg (cx , UNUSED_ROUNDING , expr . span , format ! ("used the `{method_name}` method with a whole number float") , format ! ("remove the `{method_name}` method call") , float , Applicability :: MachineApplicable ,) ; } } }
};
}
