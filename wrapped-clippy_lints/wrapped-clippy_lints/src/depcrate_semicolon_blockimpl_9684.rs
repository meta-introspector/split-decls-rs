// Generated macro for impl_9684 (impl)
macro_rules! Depcrate_semicolon_blockimpl_9684 {
() => {
// Module: crate::semicolon_block
// Provides: {"impl_9684"}
// Dependencies: {}
impl LateLintPass < '_ > for SemicolonBlock { fn check_stmt (& mut self , cx : & LateContext < '_ > , stmt : & Stmt < '_ >) { match stmt . kind { StmtKind :: Expr (Expr { kind : ExprKind :: Block (block , _) , .. }) if ! block . span . from_expansion () && stmt . span . contains (block . span) => { if block . expr . is_none () && let [.. , stmt] = block . stmts && let StmtKind :: Semi (expr) = stmt . kind { self . semicolon_outside_block (cx , block , expr) ; } } , StmtKind :: Semi (Expr { kind : ExprKind :: Block (block , _) , .. }) if ! block . span . from_expansion () => { let attrs = cx . tcx . hir_attrs (stmt . hir_id) ; if ! attrs . is_empty () && ! cx . tcx . features () . stmt_expr_attributes () { return ; } if let Some (tail) = block . expr { self . semicolon_inside_block (cx , block , tail , stmt . span) ; } } , _ => () , } } }
};
}
