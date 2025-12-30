// Generated macro for impl_4089 (impl)
macro_rules! Depcrate_manual_clampimpl_4089 {
() => {
// Module: crate::manual_clamp
// Provides: {"impl_4089"}
// Dependencies: {}
impl Clone for MaybeBorrowedStmtKind < '_ > { fn clone (& self) -> Self { match self { Self :: Borrowed (t) => Self :: Borrowed (t) , Self :: Owned (StmtKind :: Expr (e)) => Self :: Owned (StmtKind :: Expr (e)) , Self :: Owned (_) => unreachable ! ("Owned should only ever contain a StmtKind::Expr.") , } } }
};
}
