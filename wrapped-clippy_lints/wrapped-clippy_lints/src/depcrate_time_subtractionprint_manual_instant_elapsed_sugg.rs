// Generated macro for print_manual_instant_elapsed_sugg (function)
macro_rules! Depcrate_time_subtractionprint_manual_instant_elapsed_sugg {
() => {
// Module: crate::time_subtraction
// Provides: {"print_manual_instant_elapsed_sugg"}
// Dependencies: {}
fn print_manual_instant_elapsed_sugg (cx : & LateContext < '_ > , expr : & Expr < '_ > , sugg : Sugg < '_ >) { span_lint_and_sugg (cx , MANUAL_INSTANT_ELAPSED , expr . span , "manual implementation of `Instant::elapsed`" , "try" , format ! ("{}.elapsed()" , sugg . maybe_paren ()) , Applicability :: MachineApplicable ,) ; }
};
}
