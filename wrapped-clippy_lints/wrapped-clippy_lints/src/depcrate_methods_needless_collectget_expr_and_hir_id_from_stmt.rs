// Generated macro for get_expr_and_hir_id_from_stmt (function)
macro_rules! Depcrate_methods_needless_collectget_expr_and_hir_id_from_stmt {
() => {
// Module: crate::methods::needless_collect
// Provides: {"get_expr_and_hir_id_from_stmt"}
// Dependencies: {}
fn get_expr_and_hir_id_from_stmt < 'v > (stmt : & 'v Stmt < 'v >) -> Option < (& 'v Expr < 'v > , Option < HirId >) > { match stmt . kind { StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => Some ((expr , None)) , StmtKind :: Item (..) => None , StmtKind :: Let (LetStmt { init , pat , .. }) => { if let PatKind :: Binding (_ , hir_id , ..) = pat . kind { init . map (| init_expr | (init_expr , Some (hir_id))) } else { init . map (| init_expr | (init_expr , None)) } } , } }
};
}
