// Generated macro for block_stmt_with_last (function)
macro_rules! Depcrate_manual_clampblock_stmt_with_last {
() => {
// Module: crate::manual_clamp
// Provides: {"block_stmt_with_last"}
// Dependencies: {}
fn block_stmt_with_last < 'tcx > (block : & 'tcx Block < 'tcx >) -> impl Iterator < Item = MaybeBorrowedStmtKind < 'tcx > > { block . stmts . iter () . map (| s | MaybeBorrowedStmtKind :: Borrowed (& s . kind)) . chain (block . expr . as_ref () . map (| e | MaybeBorrowedStmtKind :: Owned (StmtKind :: Expr (e))) ,) }
};
}
