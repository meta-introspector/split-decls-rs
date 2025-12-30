// Generated macro for walk_block (function)
macro_rules! Depcrate_intravisitwalk_block {
() => {
// Module: crate::intravisit
// Provides: {"walk_block"}
// Dependencies: {}
pub fn walk_block < 'v , V : Visitor < 'v > > (visitor : & mut V , block : & 'v Block < 'v >) -> V :: Result { let Block { stmts , expr , hir_id , rules : _ , span : _ , targeted_by_break : _ } = block ; try_visit ! (visitor . visit_id (* hir_id)) ; walk_list ! (visitor , visit_stmt , * stmts) ; visit_opt ! (visitor , visit_expr , * expr) ; V :: Result :: output () }
};
}
