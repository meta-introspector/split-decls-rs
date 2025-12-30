// Generated macro for walk_param (function)
macro_rules! Depcrate_intravisitwalk_param {
() => {
// Module: crate::intravisit
// Provides: {"walk_param"}
// Dependencies: {}
pub fn walk_param < 'v , V : Visitor < 'v > > (visitor : & mut V , param : & 'v Param < 'v >) -> V :: Result { let Param { hir_id , pat , ty_span : _ , span : _ } = param ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_pat (pat) }
};
}
