// Generated macro for check (function)
macro_rules! Depcrate_methods_or_then_unwrapcheck {
() => {
// Module: crate::methods::or_then_unwrap
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , unwrap_expr : & Expr < '_ > , recv : & 'tcx Expr < 'tcx > , or_arg : & 'tcx Expr < '_ > , or_span : Span ,) { let ty = cx . typeck_results () . expr_ty (recv) ; let title ; let or_arg_content : Span ; if ty . is_diag_item (cx , sym :: Option) { title = "found `.or(Some(…)).unwrap()`" ; if let Some (content) = get_content_if_ctor_matches (cx , or_arg , LangItem :: OptionSome) { or_arg_content = content ; } else { return ; } } else if ty . is_diag_item (cx , sym :: Result) { title = "found `.or(Ok(…)).unwrap()`" ; if let Some (content) = get_content_if_ctor_matches (cx , or_arg , LangItem :: ResultOk) { or_arg_content = content ; } else { return ; } } else { return ; } let mut applicability = Applicability :: MachineApplicable ; let suggestion = format ! ("unwrap_or({})" , snippet_with_applicability (cx , or_arg_content , ".." , & mut applicability)) ; span_lint_and_sugg (cx , OR_THEN_UNWRAP , unwrap_expr . span . with_lo (or_span . lo ()) , title , "try" , suggestion , applicability ,) ; }
};
}
