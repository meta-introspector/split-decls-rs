// Generated macro for print_lint_and_sugg (function)
macro_rules! Depcrate_implicit_saturating_subprint_lint_and_sugg {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"print_lint_and_sugg"}
// Dependencies: {}
fn print_lint_and_sugg (cx : & LateContext < '_ > , var_name : Symbol , expr : & Expr < '_ >) { span_lint_and_sugg (cx , IMPLICIT_SATURATING_SUB , expr . span , "implicitly performing saturating subtraction" , "try" , format ! ("{var_name} = {var_name}.saturating_sub({});" , '1') , Applicability :: MachineApplicable ,) ; }
};
}
