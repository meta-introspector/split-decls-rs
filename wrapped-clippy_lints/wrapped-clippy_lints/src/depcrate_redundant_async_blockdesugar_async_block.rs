// Generated macro for desugar_async_block (function)
macro_rules! Depcrate_redundant_async_blockdesugar_async_block {
() => {
// Module: crate::redundant_async_block
// Provides: {"desugar_async_block"}
// Dependencies: {}
# [doc = " If `expr` is a desugared `async` block, return the original expression if it does not capture"] # [doc = " any variable by ref."] fn desugar_async_block < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Closure (Closure { body , def_id , kind , .. }) = expr . kind && let body = cx . tcx . hir_body (* body) && matches ! (kind , ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , CoroutineSource :: Block))) { cx . typeck_results () . closure_min_captures . get (def_id) . is_none_or (| m | { m . values () . all (| places | { places . iter () . all (| place | matches ! (place . info . capture_kind , UpvarCapture :: ByValue)) }) }) . then_some (body . value) } else { None } }
};
}
