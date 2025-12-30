// Generated macro for walk_opaque_ty (function)
macro_rules! Depcrate_intravisitwalk_opaque_ty {
() => {
// Module: crate::intravisit
// Provides: {"walk_opaque_ty"}
// Dependencies: {}
pub fn walk_opaque_ty < 'v , V : Visitor < 'v > > (visitor : & mut V , opaque : & 'v OpaqueTy < 'v >) -> V :: Result { let & OpaqueTy { hir_id , def_id : _ , bounds , origin : _ , span : _ } = opaque ; try_visit ! (visitor . visit_id (hir_id)) ; walk_list ! (visitor , visit_param_bound , bounds) ; V :: Result :: output () }
};
}
