// Generated macro for impl_3371 (impl)
macro_rules! Depcrate_let_if_seqimpl_3371 {
() => {
// Module: crate::let_if_seq
// Provides: {"impl_3371"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for LetIfSeq { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx hir :: Block < '_ >) { for [stmt , next] in block . stmts . array_windows :: < 2 > () { if let hir :: StmtKind :: Expr (if_) = next . kind { check_block_inner (cx , stmt , if_) ; } } if let Some (expr) = block . expr && let Some (stmt) = block . stmts . last () { check_block_inner (cx , stmt , expr) ; } } }
};
}
