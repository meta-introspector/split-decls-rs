// Generated macro for walk_lifetime (function)
macro_rules! Depcrate_intravisitwalk_lifetime {
() => {
// Module: crate::intravisit
// Provides: {"walk_lifetime"}
// Dependencies: {}
pub fn walk_lifetime < 'v , V : Visitor < 'v > > (visitor : & mut V , lifetime : & 'v Lifetime) -> V :: Result { let Lifetime { hir_id , ident , kind : _ , source : _ , syntax : _ } = lifetime ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_ident (* ident) }
};
}
