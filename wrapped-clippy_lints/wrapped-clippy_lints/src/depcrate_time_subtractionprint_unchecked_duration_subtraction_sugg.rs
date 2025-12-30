// Generated macro for print_unchecked_duration_subtraction_sugg (function)
macro_rules! Depcrate_time_subtractionprint_unchecked_duration_subtraction_sugg {
() => {
// Module: crate::time_subtraction
// Provides: {"print_unchecked_duration_subtraction_sugg"}
// Dependencies: {}
fn print_unchecked_duration_subtraction_sugg (cx : & LateContext < '_ > , left_expr : & Expr < '_ > , right_expr : & Expr < '_ > , expr : & Expr < '_ > ,) { span_lint_and_then (cx , UNCHECKED_TIME_SUBTRACTION , expr . span , "unchecked subtraction of a `Duration`" , | diag | { if ! is_chained_time_subtraction (cx , left_expr) { let mut applicability = Applicability :: MachineApplicable ; let left_sugg = Sugg :: hir_with_applicability (cx , left_expr , "<left>" , & mut applicability) ; let right_sugg = Sugg :: hir_with_applicability (cx , right_expr , "<right>" , & mut applicability) ; diag . span_suggestion (expr . span , "try" , format ! ("{}.checked_sub({}).unwrap()" , left_sugg . maybe_paren () , right_sugg) , applicability ,) ; } } ,) ; }
};
}
