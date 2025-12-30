// Generated macro for might_be_expanded (function)
macro_rules! Depcrate_large_stack_arraysmight_be_expanded {
() => {
// Module: crate::large_stack_arrays
// Provides: {"might_be_expanded"}
// Dependencies: {}
# [doc = " Only giving help messages if the expr does not contains macro expanded codes."] fn might_be_expanded < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> bool { # [doc = " Check if the span of `ConstArg` of a repeat expression is within the expr's span,"] # [doc = " if not, meaning this repeat expr is definitely from some proc-macro."] # [doc = ""] # [doc = " This is a fail-safe to a case where even the `is_from_proc_macro` is unable to determain the"] # [doc = " correct result."] fn repeat_expr_might_be_expanded (expr : & Expr < '_ >) -> bool { let ExprKind :: Repeat (_ , len_ct) = expr . kind else { return false ; } ; ! expr . span . contains (len_ct . span ()) } expr . span . from_expansion () || is_from_proc_macro (cx , expr) || repeat_expr_might_be_expanded (expr) }
};
}
