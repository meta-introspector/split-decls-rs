// Generated macro for impl_3237 (impl)
macro_rules! Depcrate_large_futuresimpl_3237 {
() => {
// Module: crate::large_futures
// Provides: {"impl_3237"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for LargeFuture { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let ExprKind :: Match (scrutinee , _ , MatchSource :: AwaitDesugar) = expr . kind && let ExprKind :: Call (func , [arg]) = scrutinee . kind && let ExprKind :: Path (qpath) = func . kind && cx . tcx . qpath_is_lang_item (qpath , LangItem :: IntoFutureIntoFuture) && ! expr . span . from_expansion () && let ty = cx . typeck_results () . expr_ty (arg) && let Some (future_trait_def_id) = cx . tcx . lang_items () . future_trait () && implements_trait (cx , ty , future_trait_def_id , & []) && let Ok (layout) = cx . tcx . layout_of (cx . typing_env () . as_query_input (ty)) && let size = layout . layout . size () && size >= Size :: from_bytes (self . future_size_threshold) { span_lint_and_sugg (cx , LARGE_FUTURES , arg . span , format ! ("large future with a size of {} bytes" , size . bytes ()) , "consider `Box::pin` on it" , format ! ("Box::pin({})" , snippet (cx , arg . span , "..")) , Applicability :: Unspecified ,) ; } } }
};
}
