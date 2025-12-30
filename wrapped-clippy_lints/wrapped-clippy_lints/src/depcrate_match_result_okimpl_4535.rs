// Generated macro for impl_4535 (impl)
macro_rules! Depcrate_match_result_okimpl_4535 {
() => {
// Module: crate::match_result_ok
// Provides: {"impl_4535"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MatchResultOk { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let (let_pat , let_expr , ifwhile) = if let Some (higher :: IfLet { let_pat , let_expr , .. }) = higher :: IfLet :: hir (cx , expr) { (let_pat , let_expr , "if") } else if let Some (higher :: WhileLet { let_pat , let_expr , .. }) = higher :: WhileLet :: hir (expr) { (let_pat , let_expr , "while") } else { return ; } ; if let ExprKind :: MethodCall (ok_path , recv , [] , ..) = let_expr . kind && ok_path . ident . name == sym :: ok && cx . typeck_results () . expr_ty (recv) . is_diag_item (cx , sym :: Result) && let Some ([ok_pat]) = as_some_pattern (cx , let_pat) && let ctxt = expr . span . ctxt () && let_expr . span . ctxt () == ctxt && let_pat . span . ctxt () == ctxt { let mut applicability = Applicability :: MachineApplicable ; let some_expr_string = snippet_with_context (cx , ok_pat . span , ctxt , "" , & mut applicability) . 0 ; let trimmed_ok = snippet_with_context (cx , recv . span , ctxt , "" , & mut applicability) . 0 ; let sugg = format ! ("{ifwhile} let Ok({some_expr_string}) = {}" , trimmed_ok . trim () . trim_end_matches ('.') ,) ; span_lint_and_sugg (cx , MATCH_RESULT_OK , expr . span . with_hi (let_expr . span . hi ()) , "matching on `Some` with `ok()` is redundant" , format ! ("consider matching on `Ok({some_expr_string})` and removing the call to `ok` instead") , sugg , applicability ,) ; } } }
};
}
