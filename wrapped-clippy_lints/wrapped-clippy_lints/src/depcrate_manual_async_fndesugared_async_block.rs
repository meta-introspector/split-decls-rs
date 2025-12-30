// Generated macro for desugared_async_block (function)
macro_rules! Depcrate_manual_async_fndesugared_async_block {
() => {
// Module: crate::manual_async_fn
// Provides: {"desugared_async_block"}
// Dependencies: {}
fn desugared_async_block < 'tcx > (cx : & LateContext < 'tcx > , block : & 'tcx Block < 'tcx >) -> Option < & 'tcx Body < 'tcx > > { if let Some (& Expr { kind : ExprKind :: Closure (& Closure { kind , body , .. }) , .. }) = block . expr && let ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , CoroutineSource :: Block)) = kind { return Some (cx . tcx . hir_body (body)) ; } None }
};
}
