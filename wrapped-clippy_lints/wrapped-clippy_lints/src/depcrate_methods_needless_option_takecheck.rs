// Generated macro for check (function)
macro_rules! Depcrate_methods_needless_option_takecheck {
() => {
// Module: crate::methods::needless_option_take
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , recv : & 'tcx Expr < '_ >) { if ! recv . is_syntactic_place_expr () && is_expr_option (cx , recv) && let Some (function_name) = source_of_temporary_value (recv) { span_lint_and_then (cx , NEEDLESS_OPTION_TAKE , expr . span , "called `Option::take()` on a temporary value" , | diag | { diag . note (format ! ("`{function_name}` creates a temporary value, so calling take() has no effect")) ; diag . span_suggestion (expr . span . with_lo (recv . span . hi ()) , "remove" , "" , Applicability :: MachineApplicable ,) ; } ,) ; } }
};
}
