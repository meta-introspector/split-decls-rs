// Generated macro for check_as_ptr (function)
macro_rules! Depcrate_methods_manual_c_str_literalscheck_as_ptr {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"check_as_ptr"}
// Dependencies: {}
# [doc = " Checks:"] # [doc = " - `b\"...\".as_ptr()`"] # [doc = " - `b\"...\".as_ptr().cast()`"] # [doc = " - `\"...\".as_ptr()`"] # [doc = " - `\"...\".as_ptr().cast()`"] # [doc = ""] # [doc = " Iff the parent call of `.cast()` isn't `CStr::from_ptr`, to avoid linting twice."] pub (super) fn check_as_ptr < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , receiver : & 'tcx Expr < 'tcx > , msrv : Msrv ,) { if let ExprKind :: Lit (lit) = receiver . kind && let LitKind :: ByteStr (_ , StrStyle :: Cooked) | LitKind :: Str (_ , StrStyle :: Cooked) = lit . node && cx . tcx . sess . edition () >= Edition2021 && let casts_removed = peel_ptr_cast_ancestors (cx , expr) && ! get_parent_expr (cx , casts_removed) . is_some_and (| parent | matches ! (parent . kind , ExprKind :: Call (func , _) if is_c_str_function (cx , func) . is_some ()) ,) && let Some (sugg) = rewrite_as_cstr (cx , lit . span) && msrv . meets (cx , msrvs :: C_STR_LITERALS) { span_lint_and_sugg (cx , MANUAL_C_STR_LITERALS , receiver . span , "manually constructing a nul-terminated string" , r#"use a `c""` literal"# , sugg , Applicability :: HasPlaceholders ,) ; } }
};
}
