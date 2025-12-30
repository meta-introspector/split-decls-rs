// Generated macro for impl_3403 (impl)
macro_rules! Depcrate_lines_filter_map_okimpl_3403 {
() => {
// Module: crate::lines_filter_map_ok
// Provides: {"impl_3403"}
// Dependencies: {}
impl LateLintPass < '_ > for LinesFilterMapOk { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: MethodCall (fm_method , fm_receiver , fm_args , fm_span) = expr . kind && is_trait_method (cx , expr , sym :: Iterator) && let fm_method_name = fm_method . ident . name && matches ! (fm_method_name , sym :: filter_map | sym :: flat_map | sym :: flatten) && is_type_diagnostic_item (cx , cx . typeck_results () . expr_ty_adjusted (fm_receiver) , sym :: IoLines) && should_lint (cx , fm_args , fm_method_name) && self . msrv . meets (cx , msrvs :: MAP_WHILE) { span_lint_and_then (cx , LINES_FILTER_MAP_OK , fm_span , format ! ("`{fm_method_name}()` will run forever if the iterator repeatedly produces an `Err`" ,) , | diag | { diag . span_note (fm_receiver . span , "this expression returning a `std::io::Lines` may produce an infinite number of `Err` in case of a read error") ; diag . span_suggestion (fm_span , "replace with" , "map_while(Result::ok)" , Applicability :: MaybeIncorrect ,) ; } ,) ; } } }
};
}
