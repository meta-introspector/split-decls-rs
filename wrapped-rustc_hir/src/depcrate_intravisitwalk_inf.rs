// Generated macro for walk_inf (function)
macro_rules! Depcrate_intravisitwalk_inf {
() => {
// Module: crate::intravisit
// Provides: {"walk_inf"}
// Dependencies: {}
pub fn walk_inf < 'v , V : Visitor < 'v > > (visitor : & mut V , inf : & 'v InferArg) -> V :: Result { let InferArg { hir_id , span : _ } = inf ; visitor . visit_id (* hir_id) }
};
}
