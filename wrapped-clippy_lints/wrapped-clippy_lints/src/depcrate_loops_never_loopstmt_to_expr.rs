// Generated macro for stmt_to_expr (function)
macro_rules! Depcrate_loops_never_loopstmt_to_expr {
() => {
// Module: crate::loops::never_loop
// Provides: {"stmt_to_expr"}
// Dependencies: {}
fn stmt_to_expr < 'tcx > (stmt : & Stmt < 'tcx >) -> Option < (& 'tcx Expr < 'tcx > , Option < & 'tcx Block < 'tcx > >) > { match stmt . kind { StmtKind :: Semi (e) | StmtKind :: Expr (e) => Some ((e , None)) , StmtKind :: Let (local) => local . init . map (| init | (init , local . els)) , StmtKind :: Item (..) => None , } }
};
}
