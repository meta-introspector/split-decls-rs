// Generated macro for check (function)
macro_rules! Depcrate_methods_bytes_count_to_lencheck {
() => {
// Module: crate::methods::bytes_count_to_len
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , count_recv : & 'tcx hir :: Expr < '_ > , bytes_recv : & 'tcx hir :: Expr < '_ > ,) { if let Some (bytes_id) = cx . typeck_results () . type_dependent_def_id (count_recv . hir_id) && let Some (impl_id) = cx . tcx . impl_of_assoc (bytes_id) && cx . tcx . type_of (impl_id) . instantiate_identity () . is_str () && let ty = cx . typeck_results () . expr_ty (bytes_recv) . peel_refs () && (ty . is_str () || ty . is_lang_item (cx , hir :: LangItem :: String)) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , BYTES_COUNT_TO_LEN , expr . span , "using long and hard to read `.bytes().count()`" , "consider calling `.len()` instead" , format ! ("{}.len()" , snippet_with_applicability (cx , bytes_recv . span , ".." , & mut applicability)) , applicability ,) ; } }
};
}
