// Generated macro for walk_arm (function)
macro_rules! Depcrate_intravisitwalk_arm {
() => {
// Module: crate::intravisit
// Provides: {"walk_arm"}
// Dependencies: {}
pub fn walk_arm < 'v , V : Visitor < 'v > > (visitor : & mut V , arm : & 'v Arm < 'v >) -> V :: Result { let Arm { hir_id , span : _ , pat , guard , body } = arm ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_pat (* pat)) ; visit_opt ! (visitor , visit_expr , * guard) ; visitor . visit_expr (* body) }
};
}
