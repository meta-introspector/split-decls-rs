// Generated macro for expr_parent_is_else (function)
macro_rules! Depcrate_if_let_rescopeexpr_parent_is_else {
() => {
// Module: crate::if_let_rescope
// Provides: {"expr_parent_is_else"}
// Dependencies: {}
fn expr_parent_is_else (tcx : TyCtxt < '_ > , hir_id : hir :: HirId) -> bool { let Some ((_ , hir :: Node :: Expr (expr))) = tcx . hir_parent_iter (hir_id) . next () else { return false ; } ; let hir :: ExprKind :: If (_cond , _conseq , Some (alt)) = expr . kind else { return false } ; alt . hir_id == hir_id }
};
}
