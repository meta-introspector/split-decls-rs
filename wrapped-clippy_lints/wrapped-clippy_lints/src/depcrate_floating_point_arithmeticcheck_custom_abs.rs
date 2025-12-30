// Generated macro for check_custom_abs (function)
macro_rules! Depcrate_floating_point_arithmeticcheck_custom_abs {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"check_custom_abs"}
// Dependencies: {}
fn check_custom_abs (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let Some (higher :: If { cond , then , r#else : Some (r#else) , }) = higher :: If :: hir (expr) && let if_body_expr = peel_blocks (then) && let else_body_expr = peel_blocks (r#else) && let Some ((if_expr_positive , body)) = are_negated (cx , if_body_expr , else_body_expr) { let positive_abs_sugg = ("manual implementation of `abs` method" , format ! ("{}.abs()" , Sugg :: hir (cx , body , "..") . maybe_paren ()) ,) ; let negative_abs_sugg = ("manual implementation of negation of `abs` method" , format ! ("-{}.abs()" , Sugg :: hir (cx , body , "..") . maybe_paren ()) ,) ; let sugg = if is_testing_positive (cx , cond , body) { if if_expr_positive { positive_abs_sugg } else { negative_abs_sugg } } else if is_testing_negative (cx , cond , body) { if if_expr_positive { negative_abs_sugg } else { positive_abs_sugg } } else { return ; } ; span_lint_and_sugg (cx , SUBOPTIMAL_FLOPS , expr . span , sugg . 0 , "try" , sugg . 1 , Applicability :: MachineApplicable ,) ; } }
};
}
