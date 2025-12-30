// Generated macro for impl_10917 (impl)
macro_rules! Depcrate_unnecessary_owned_empty_stringsimpl_10917 {
() => {
// Module: crate::unnecessary_owned_empty_strings
// Provides: {"impl_10917"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnnecessaryOwnedEmptyStrings { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Not , inner_expr) = expr . kind && let ExprKind :: Call (fun , args) = inner_expr . kind && let ExprKind :: Path (ref qpath) = fun . kind && let Some (fun_def_id) = cx . qpath_res (qpath , fun . hir_id) . opt_def_id () && let ty :: Ref (_ , inner_str , _) = cx . typeck_results () . expr_ty_adjusted (expr) . kind () && inner_str . is_str () { let fun_name = cx . tcx . get_diagnostic_name (fun_def_id) ; if fun_name == Some (sym :: string_new) { span_lint_and_sugg (cx , UNNECESSARY_OWNED_EMPTY_STRINGS , expr . span , "usage of `&String::new()` for a function expecting a `&str` argument" , "try" , "\"\"" . to_owned () , Applicability :: MachineApplicable ,) ; } else if fun_name == Some (sym :: from_fn) && let [arg] = args && let ExprKind :: Lit (spanned) = & arg . kind && let LitKind :: Str (symbol , _) = spanned . node && symbol . is_empty () && let inner_expr_type = cx . typeck_results () . expr_ty (inner_expr) && inner_expr_type . is_lang_item (cx , LangItem :: String) { span_lint_and_sugg (cx , UNNECESSARY_OWNED_EMPTY_STRINGS , expr . span , "usage of `&String::from(\"\")` for a function expecting a `&str` argument" , "try" , "\"\"" . to_owned () , Applicability :: MachineApplicable ,) ; } } } }
};
}
