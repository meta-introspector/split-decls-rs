// Generated macro for check (function)
macro_rules! Depcrate_matches_match_wild_err_armcheck {
() => {
// Module: crate::matches::match_wild_err_arm
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check < 'tcx > (cx : & LateContext < 'tcx > , ex : & Expr < 'tcx > , arms : & [Arm < 'tcx >]) { if is_in_const_context (cx) { return ; } let ex_ty = cx . typeck_results () . expr_ty (ex) . peel_refs () ; if ex_ty . is_diag_item (cx , sym :: Result) { for arm in arms { if let PatKind :: TupleStruct (ref path , inner , _) = arm . pat . kind { let path_str = rustc_hir_pretty :: qpath_to_string (& cx . tcx , path) ; if path_str == "Err" { let mut matching_wild = inner . iter () . any (is_wild) ; let mut ident_bind_name = kw :: Underscore ; if ! matching_wild { for pat in inner { if let PatKind :: Binding (_ , id , ident , None) = pat . kind && ident . as_str () . starts_with ('_') && ! is_local_used (cx , arm . body , id) { ident_bind_name = ident . name ; matching_wild = true ; } } } if matching_wild && let Some (macro_call) = root_macro_call (peel_blocks_with_stmt (arm . body) . span) && is_panic (cx , macro_call . def_id) { span_lint_and_note (cx , MATCH_WILD_ERR_ARM , arm . pat . span , format ! ("`Err({ident_bind_name})` matches all errors") , None , "match each error separately or use the error output, or use `.expect(msg)` if the error case is unreachable" ,) ; } } } } } }
};
}
