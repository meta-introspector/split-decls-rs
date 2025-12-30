// Generated macro for all_spans_after_expr (function)
macro_rules! Depcrate_loops_never_loopall_spans_after_expr {
() => {
// Module: crate::loops::never_loop
// Provides: {"all_spans_after_expr"}
// Dependencies: {}
# [doc = " Returns a Vec of all the individual spans after the highlighted expression in a block"] fn all_spans_after_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Vec < Span > { if let Node :: Stmt (stmt) = cx . tcx . parent_hir_node (expr . hir_id) { if let Node :: Block (block) = cx . tcx . parent_hir_node (stmt . hir_id) { return block . stmts . iter () . skip_while (| inner | inner . hir_id != stmt . hir_id) . map (stmt_source_span) . chain (block . expr . map (| e | e . span)) . collect () ; } return vec ! [stmt . span] ; } else if let Node :: Block (_) = cx . tcx . parent_hir_node (expr . hir_id) { return vec ! [expr . span] ; } vec ! [] }
};
}
