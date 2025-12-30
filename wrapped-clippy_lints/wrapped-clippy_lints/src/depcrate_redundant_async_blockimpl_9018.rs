// Generated macro for impl_9018 (impl)
macro_rules! Depcrate_redundant_async_blockimpl_9018 {
() => {
// Module: crate::redundant_async_block
// Provides: {"impl_9018"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for RedundantAsyncBlock { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let span = expr . span ; if ! span . in_external_macro (cx . tcx . sess . source_map ()) && let Some (body_expr) = desugar_async_block (cx , expr) && let Some (expr) = desugar_await (peel_blocks (body_expr)) && expr . span . eq_ctxt (body_expr . span) && let Some (future_trait) = cx . tcx . lang_items () . future_trait () && implements_trait (cx , cx . typeck_results () . expr_ty (expr) , future_trait , & []) && (! expr . can_have_side_effects () || desugar_async_block (cx , expr) . is_some ()) && let Some (shortened_span) = walk_span_to_context (expr . span , span . ctxt ()) { span_lint_and_sugg (cx , REDUNDANT_ASYNC_BLOCK , span , "this async expression only awaits a single future" , "you can reduce it to" , snippet (cx , shortened_span , "..") . into_owned () , Applicability :: MachineApplicable ,) ; } } }
};
}
