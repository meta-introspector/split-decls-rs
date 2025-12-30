// Generated macro for is_expr_parent_assignment (function)
macro_rules! Depcrate_ifs_branches_sharing_codeis_expr_parent_assignment {
() => {
// Module: crate::ifs::branches_sharing_code
// Provides: {"is_expr_parent_assignment"}
// Dependencies: {}
fn is_expr_parent_assignment (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let parent = cx . tcx . parent_hir_node (expr . hir_id) ; if let Node :: LetStmt (LetStmt { init : Some (e) , .. }) | Node :: Expr (Expr { kind : ExprKind :: Assign (_ , e , _) , .. }) = parent { return e . hir_id == expr . hir_id ; } false }
};
}
