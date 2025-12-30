// Generated macro for impl_4747 (impl)
macro_rules! Depcrate_matches_match_str_case_mismatchimpl_4747 {
() => {
// Module: crate::matches::match_str_case_mismatch
// Provides: {"impl_4747"}
// Dependencies: {}
impl MatchExprVisitor < '_ , '_ > { fn case_altered (& self , segment_ident : Symbol , receiver : & Expr < '_ >) -> ControlFlow < CaseMethod > { if let Some (case_method) = get_case_method (segment_ident) { let ty = self . cx . typeck_results () . expr_ty (receiver) . peel_refs () ; if is_type_lang_item (self . cx , ty , LangItem :: String) || ty . kind () == & ty :: Str { return ControlFlow :: Break (case_method) ; } } ControlFlow :: Continue (()) } }
};
}
