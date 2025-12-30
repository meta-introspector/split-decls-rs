// Generated macro for impl_11060 (impl)
macro_rules! Depcrate_unused_asyncimpl_11060 {
() => {
// Module: crate::unused_async
// Provides: {"impl_11060"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for AsyncFnVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn visit_expr (& mut self , ex : & 'tcx Expr < 'tcx >) { if let ExprKind :: Yield (_ , YieldSource :: Await { .. }) = ex . kind { if self . async_depth == 1 { self . found_await = true ; } else if self . await_in_async_block . is_none () { self . await_in_async_block = Some (ex . span) ; } } let is_async_block = matches ! (ex . kind , ExprKind :: Closure (Closure { kind : ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _)) , .. })) ; if is_async_block { self . async_depth += 1 ; } walk_expr (self , ex) ; if is_async_block { self . async_depth -= 1 ; } } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
