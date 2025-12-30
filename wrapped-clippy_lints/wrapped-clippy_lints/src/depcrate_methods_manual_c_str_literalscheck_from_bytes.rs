// Generated macro for check_from_bytes (function)
macro_rules! Depcrate_methods_manual_c_str_literalscheck_from_bytes {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"check_from_bytes"}
// Dependencies: {}
# [doc = " Checks `CStr::from_bytes_with_nul(b\"foo\\0\")`"] fn check_from_bytes (cx : & LateContext < '_ > , expr : & Expr < '_ > , arg : & Expr < '_ > , method : Symbol) { let (span , applicability) = if let Some (parent) = get_parent_expr (cx , expr) && let ExprKind :: MethodCall (method , ..) = parent . kind && [sym :: unwrap , sym :: expect] . contains (& method . ident . name) { (parent . span , Applicability :: MachineApplicable) } else if method == sym :: from_bytes_with_nul_unchecked { (expr . span , Applicability :: MachineApplicable) } else { (expr . span , Applicability :: HasPlaceholders) } ; let Some (sugg) = rewrite_as_cstr (cx , arg . span) else { return ; } ; span_lint_and_sugg (cx , MANUAL_C_STR_LITERALS , span , "calling `CStr::new` with a byte string literal" , r#"use a `c""` literal"# , sugg , applicability ,) ; }
};
}
