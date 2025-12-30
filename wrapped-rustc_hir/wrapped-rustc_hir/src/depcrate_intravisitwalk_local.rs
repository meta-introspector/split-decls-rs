// Generated macro for walk_local (function)
macro_rules! Depcrate_intravisitwalk_local {
() => {
// Module: crate::intravisit
// Provides: {"walk_local"}
// Dependencies: {}
pub fn walk_local < 'v , V : Visitor < 'v > > (visitor : & mut V , local : & 'v LetStmt < 'v >) -> V :: Result { let LetStmt { super_ : _ , pat , ty , init , els , hir_id , span : _ , source : _ } = local ; visit_opt ! (visitor , visit_expr , * init) ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_pat (* pat)) ; visit_opt ! (visitor , visit_block , * els) ; visit_opt ! (visitor , visit_ty_unambig , * ty) ; V :: Result :: output () }
};
}
