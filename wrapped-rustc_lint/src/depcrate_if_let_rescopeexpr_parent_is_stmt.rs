// Generated macro for expr_parent_is_stmt (function)
macro_rules! Depcrate_if_let_rescopeexpr_parent_is_stmt {
() => {
// Module: crate::if_let_rescope
// Provides: {"expr_parent_is_stmt"}
// Dependencies: {}
fn expr_parent_is_stmt (tcx : TyCtxt < '_ > , hir_id : hir :: HirId) -> bool { let mut parents = tcx . hir_parent_iter (hir_id) ; let stmt = match parents . next () { Some ((_ , hir :: Node :: Stmt (stmt))) => stmt , Some ((_ , hir :: Node :: Block (_) | hir :: Node :: Arm (_))) => return true , _ => return false , } ; let (hir :: StmtKind :: Semi (expr) | hir :: StmtKind :: Expr (expr)) = stmt . kind else { return false } ; expr . hir_id == hir_id }
};
}
