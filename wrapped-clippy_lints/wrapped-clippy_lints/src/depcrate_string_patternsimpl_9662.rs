// Generated macro for impl_9662 (impl)
macro_rules! Depcrate_string_patternsimpl_9662 {
() => {
// Module: crate::string_patterns
// Provides: {"impl_9662"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for StringPatterns { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if ! expr . span . from_expansion () && let ExprKind :: MethodCall (method , receiver , args , _) = expr . kind && let ty :: Ref (_ , ty , _) = cx . typeck_results () . expr_ty_adjusted (receiver) . kind () && ty . is_str () && let method_name = method . ident . name && let Some (& (_ , pos)) = PATTERN_METHODS . iter () . find (| (array_method_name , _) | * array_method_name == method_name) && let Some (arg) = args . get (pos) { check_single_char_pattern_lint (cx , arg) ; check_manual_pattern_char_comparison (cx , arg , self . msrv) ; } } }
};
}
