// Generated macro for check (function)
macro_rules! Depcrate_methods_sliced_string_as_bytescheck {
() => {
// Module: crate::methods::sliced_string_as_bytes
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ >) { if let ExprKind :: Index (indexed , index , _) = recv . kind && is_bounded_range_literal (cx , index) && let ty = cx . typeck_results () . expr_ty (indexed) . peel_refs () && (ty . is_str () || ty . is_lang_item (cx , LangItem :: String)) { let mut applicability = Applicability :: MaybeIncorrect ; let stringish = snippet_with_applicability (cx , indexed . span , "_" , & mut applicability) ; let range = snippet_with_applicability (cx , index . span , "_" , & mut applicability) ; span_lint_and_sugg (cx , SLICED_STRING_AS_BYTES , expr . span , "calling `as_bytes` after slicing a string" , "try" , format ! ("&{stringish}.as_bytes()[{range}]") , applicability ,) ; } }
};
}
