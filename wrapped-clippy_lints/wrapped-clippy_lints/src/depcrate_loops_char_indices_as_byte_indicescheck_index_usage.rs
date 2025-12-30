// Generated macro for check_index_usage (function)
macro_rules! Depcrate_loops_char_indices_as_byte_indicescheck_index_usage {
() => {
// Module: crate::loops::char_indices_as_byte_indices
// Provides: {"check_index_usage"}
// Dependencies: {}
fn check_index_usage < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , pat : & Pat < '_ > , enumerate_span : Span , chars_span : Span , chars_recv : & Expr < '_ > ,) { let Some (parent_expr) = index_consumed_at (cx , expr) else { return ; } ; let is_string_like = | ty : Ty < '_ > | ty . is_str () || ty . is_lang_item (cx , LangItem :: String) ; let message = match parent_expr . kind { ExprKind :: MethodCall (segment , recv , ..) if cx . typeck_results () . expr_ty_adjusted (recv) . peel_refs () . is_str () && BYTE_INDEX_METHODS . contains (& segment . ident . name) && eq_expr_value (cx , chars_recv , recv) => { "passing a character position to a method that expects a byte index" } , ExprKind :: Index (target , ..) if is_string_like (cx . typeck_results () . expr_ty_adjusted (target) . peel_refs ()) && eq_expr_value (cx , chars_recv , target) => { "indexing into a string with a character position where a byte index is expected" } , _ => return , } ; span_lint_hir_and_then (cx , CHAR_INDICES_AS_BYTE_INDICES , expr . hir_id , expr . span , message , | diag | { diag . note ("a character can take up more than one byte, so they are not interchangeable") . span_note (MultiSpan :: from_spans (vec ! [pat . span , enumerate_span]) , "position comes from the enumerate iterator" ,) . span_suggestion_verbose (chars_span . to (enumerate_span) , "consider using `.char_indices()` instead" , "char_indices()" , Applicability :: MaybeIncorrect ,) ; } ,) ; }
};
}
