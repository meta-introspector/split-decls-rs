// Generated macro for impl_4496 (impl)
macro_rules! Depcrate_match_result_okimpl_4496 {
() => {
// Module: crate::match_result_ok
// Provides: {"impl_4496"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MatchResultOk { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let (let_pat , let_expr , ifwhile) = if let Some (higher :: IfLet { let_pat , let_expr , .. }) = higher :: IfLet :: hir (cx , expr) { (let_pat , let_expr , "if") } else if let Some (higher :: WhileLet { let_pat , let_expr , .. }) = higher :: WhileLet :: hir (expr) { (let_pat , let_expr , "while") } else { return ; } ; if let ExprKind :: MethodCall (ok_path , recv , [] , ..) = let_expr . kind && let PatKind :: TupleStruct (ref pat_path , [ok_pat] , _) = let_pat . kind && ok_path . ident . name == sym :: ok && is_type_diagnostic_item (cx , cx . typeck_results () . expr_ty (recv) , sym :: Result) && is_res_lang_ctor (cx , cx . qpath_res (pat_path , let_pat . hir_id) , LangItem :: OptionSome) && let ctxt = expr . span . ctxt () && let_expr . span . ctxt () == ctxt && let_pat . span . ctxt () == ctxt { let mut applicability = Applicability :: MachineApplicable ; let some_expr_string = snippet_with_context (cx , ok_pat . span , ctxt , "" , & mut applicability) . 0 ; let trimmed_ok = snippet_with_context (cx , recv . span , ctxt , "" , & mut applicability) . 0 ; let sugg = format ! ("{ifwhile} let Ok({some_expr_string}) = {}" , trimmed_ok . trim () . trim_end_matches ('.') ,) ; span_lint_and_sugg (cx , MATCH_RESULT_OK , expr . span . with_hi (let_expr . span . hi ()) , "matching on `Some` with `ok()` is redundant" , format ! ("consider matching on `Ok({some_expr_string})` and removing the call to `ok` instead") , sugg , applicability ,) ; } } }
};
}
