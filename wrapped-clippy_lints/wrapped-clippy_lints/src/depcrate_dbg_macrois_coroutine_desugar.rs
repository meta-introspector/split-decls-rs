// Generated macro for is_coroutine_desugar (function)
macro_rules! Depcrate_dbg_macrois_coroutine_desugar {
() => {
// Module: crate::dbg_macro
// Provides: {"is_coroutine_desugar"}
// Dependencies: {}
fn is_coroutine_desugar (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Closure (Closure { kind : ClosureKind :: Coroutine (CoroutineKind :: Desugared (..)) | ClosureKind :: CoroutineClosure (..) , .. })) }
};
}
