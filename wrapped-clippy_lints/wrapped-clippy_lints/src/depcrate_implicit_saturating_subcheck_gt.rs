// Generated macro for check_gt (function)
macro_rules! Depcrate_implicit_saturating_subcheck_gt {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"check_gt"}
// Dependencies: {}
# [expect (clippy :: too_many_arguments)] fn check_gt (cx : & LateContext < '_ > , condition_span : Span , expr_span : Span , big_expr : & Expr < '_ > , little_expr : & Expr < '_ > , if_block : & Expr < '_ > , else_block : & Expr < '_ > , msrv : Msrv , is_composited : bool ,) { if is_side_effect_free (cx , big_expr) && is_side_effect_free (cx , little_expr) { check_subtraction (cx , condition_span , expr_span , big_expr , little_expr , if_block , else_block , msrv , is_composited ,) ; } }
};
}
