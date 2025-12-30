// Generated macro for check (function)
macro_rules! Depcrate_methods_iter_filtercheck {
() => {
// Module: crate::methods::iter_filter
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , filter_arg : & hir :: Expr < '_ > , filter_span : Span) { match expression_type (cx , expr , filter_arg , filter_span) { None => () , Some (FilterType :: IsOk) => span_lint_and_sugg (cx , ITER_FILTER_IS_OK , filter_span . with_hi (expr . span . hi ()) , "`filter` for `is_ok` on iterator over `Result`s" , "consider using `flatten` instead" , reindent_multiline ("flatten()" , true , indent_of (cx , filter_span)) , Applicability :: HasPlaceholders ,) , Some (FilterType :: IsSome) => span_lint_and_sugg (cx , ITER_FILTER_IS_SOME , filter_span . with_hi (expr . span . hi ()) , "`filter` for `is_some` on iterator over `Option`" , "consider using `flatten` instead" , reindent_multiline ("flatten()" , true , indent_of (cx , filter_span)) , Applicability :: HasPlaceholders ,) , } }
};
}
