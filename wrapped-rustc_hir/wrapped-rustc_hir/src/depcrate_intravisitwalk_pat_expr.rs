// Generated macro for walk_pat_expr (function)
macro_rules! Depcrate_intravisitwalk_pat_expr {
() => {
// Module: crate::intravisit
// Provides: {"walk_pat_expr"}
// Dependencies: {}
pub fn walk_pat_expr < 'v , V : Visitor < 'v > > (visitor : & mut V , expr : & 'v PatExpr < 'v >) -> V :: Result { let PatExpr { hir_id , span , kind } = expr ; try_visit ! (visitor . visit_id (* hir_id)) ; match kind { PatExprKind :: Lit { lit , negated } => visitor . visit_lit (* hir_id , * lit , * negated) , PatExprKind :: ConstBlock (c) => visitor . visit_inline_const (c) , PatExprKind :: Path (qpath) => visitor . visit_qpath (qpath , * hir_id , * span) , } }
};
}
