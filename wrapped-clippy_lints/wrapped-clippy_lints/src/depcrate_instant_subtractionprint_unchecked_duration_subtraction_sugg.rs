// Generated macro for print_unchecked_duration_subtraction_sugg (function)
macro_rules! Depcrate_instant_subtractionprint_unchecked_duration_subtraction_sugg {
() => {
// Module: crate::instant_subtraction
// Provides: {"print_unchecked_duration_subtraction_sugg"}
// Dependencies: {}
fn print_unchecked_duration_subtraction_sugg (cx : & LateContext < '_ > , left_expr : & Expr < '_ > , right_expr : & Expr < '_ > , expr : & Expr < '_ > ,) { let mut applicability = Applicability :: MachineApplicable ; let ctxt = expr . span . ctxt () ; let left_expr = snippet_with_context (cx , left_expr . span , ctxt , "<instant>" , & mut applicability) . 0 ; let right_expr = snippet_with_context (cx , right_expr . span , ctxt , "<duration>" , & mut applicability) . 0 ; span_lint_and_sugg (cx , UNCHECKED_DURATION_SUBTRACTION , expr . span , "unchecked subtraction of a 'Duration' from an 'Instant'" , "try" , format ! ("{left_expr}.checked_sub({right_expr}).unwrap()") , applicability ,) ; }
};
}
