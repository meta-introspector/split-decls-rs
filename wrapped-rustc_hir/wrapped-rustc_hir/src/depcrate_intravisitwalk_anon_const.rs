// Generated macro for walk_anon_const (function)
macro_rules! Depcrate_intravisitwalk_anon_const {
() => {
// Module: crate::intravisit
// Provides: {"walk_anon_const"}
// Dependencies: {}
pub fn walk_anon_const < 'v , V : Visitor < 'v > > (visitor : & mut V , constant : & 'v AnonConst) -> V :: Result { let AnonConst { hir_id , def_id : _ , body , span : _ } = constant ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_nested_body (* body) }
};
}
