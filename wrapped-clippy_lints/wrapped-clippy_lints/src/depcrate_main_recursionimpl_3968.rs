// Generated macro for impl_3968 (impl)
macro_rules! Depcrate_main_recursionimpl_3968 {
() => {
// Module: crate::main_recursion
// Provides: {"impl_3968"}
// Dependencies: {}
impl LateLintPass < '_ > for MainRecursion { fn check_crate (& mut self , cx : & LateContext < '_ >) { self . has_no_std_attr = is_no_std_crate (cx) ; } fn check_expr_post (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if self . has_no_std_attr { return ; } if let ExprKind :: Call (func , []) = & expr . kind && let ExprKind :: Path (QPath :: Resolved (_ , path)) = & func . kind && let Some (def_id) = path . res . opt_def_id () && is_entrypoint_fn (cx , def_id) { span_lint_and_help (cx , MAIN_RECURSION , func . span , format ! ("recursing into entrypoint `{}`" , snippet (cx , func . span , "main")) , None , "consider using another function for this recursion" ,) ; } } }
};
}
