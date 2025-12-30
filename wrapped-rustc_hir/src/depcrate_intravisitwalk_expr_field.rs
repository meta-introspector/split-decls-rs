// Generated macro for walk_expr_field (function)
macro_rules! Depcrate_intravisitwalk_expr_field {
() => {
// Module: crate::intravisit
// Provides: {"walk_expr_field"}
// Dependencies: {}
pub fn walk_expr_field < 'v , V : Visitor < 'v > > (visitor : & mut V , field : & 'v ExprField < 'v >) -> V :: Result { let ExprField { hir_id , ident , expr , span : _ , is_shorthand : _ } = field ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_ident (* ident)) ; visitor . visit_expr (* expr) }
};
}
