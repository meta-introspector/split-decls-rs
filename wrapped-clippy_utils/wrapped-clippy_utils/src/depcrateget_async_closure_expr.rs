// Generated macro for get_async_closure_expr (function)
macro_rules! Depcrateget_async_closure_expr {
() => {
// Module: crate
// Provides: {"get_async_closure_expr"}
// Dependencies: {}
# [doc = " Peels away all the compiler generated code surrounding the body of an async closure."] pub fn get_async_closure_expr < 'tcx > (tcx : TyCtxt < 'tcx > , expr : & Expr < '_ >) -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Closure (& Closure { body , kind : hir :: ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _)) , .. }) = expr . kind && let ExprKind :: Block (Block { expr : Some (Expr { kind : ExprKind :: DropTemps (inner_expr) , .. }) , .. } , _ ,) = tcx . hir_body (body) . value . kind { Some (inner_expr) } else { None } }
};
}
