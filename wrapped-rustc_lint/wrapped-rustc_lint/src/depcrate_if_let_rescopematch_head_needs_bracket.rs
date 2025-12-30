// Generated macro for match_head_needs_bracket (function)
macro_rules! Depcrate_if_let_rescopematch_head_needs_bracket {
() => {
// Module: crate::if_let_rescope
// Provides: {"match_head_needs_bracket"}
// Dependencies: {}
fn match_head_needs_bracket (tcx : TyCtxt < '_ > , expr : & hir :: Expr < '_ >) -> bool { expr_parent_is_else (tcx , expr . hir_id) && matches ! (expr . kind , hir :: ExprKind :: If (..)) }
};
}
