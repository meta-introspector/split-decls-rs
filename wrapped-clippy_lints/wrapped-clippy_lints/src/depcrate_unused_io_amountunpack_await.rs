// Generated macro for unpack_await (function)
macro_rules! Depcrate_unused_io_amountunpack_await {
() => {
// Module: crate::unused_io_amount
// Provides: {"unpack_await"}
// Dependencies: {}
# [doc = " If `expr` is an (e).await, return the inner expression \"e\" that's being"] # [doc = " waited on.  Otherwise return None."] fn unpack_await < 'a > (cx : & LateContext < '_ > , expr : & 'a hir :: Expr < 'a >) -> & 'a hir :: Expr < 'a > { if let ExprKind :: Match (expr , _ , hir :: MatchSource :: AwaitDesugar) = expr . kind && let ExprKind :: Call (func , [arg_0]) = expr . kind && let ExprKind :: Path (qpath) = func . kind && cx . tcx . qpath_is_lang_item (qpath , hir :: LangItem :: IntoFutureIntoFuture) { return arg_0 ; } expr }
};
}
