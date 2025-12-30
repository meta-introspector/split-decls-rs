// Generated macro for impl_260 (impl)
macro_rules! Depcrate_expanded_nodes_expanded_stmtimpl_260 {
() => {
// Module: crate::expanded_nodes::expanded_stmt
// Provides: {"impl_260"}
// Dependencies: {}
impl ExpandedStmtHasAttrs for ExpandedStmt { fn visit_stmt_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { match & mut self . 0 . kind { ast :: StmtKind :: Let (local) => f (& mut local . attrs) , ast :: StmtKind :: Item (item) => f (& mut item . attrs) , ast :: StmtKind :: Expr (expr) | ast :: StmtKind :: Semi (expr) => f (& mut expr . attrs) , ast :: StmtKind :: MacCall (mac) => f (& mut mac . attrs) , ast :: StmtKind :: Empty => { } , } } }
};
}
