// Generated macro for walk_pat_field (function)
macro_rules! Depcrate_intravisitwalk_pat_field {
() => {
// Module: crate::intravisit
// Provides: {"walk_pat_field"}
// Dependencies: {}
pub fn walk_pat_field < 'v , V : Visitor < 'v > > (visitor : & mut V , field : & 'v PatField < 'v >) -> V :: Result { let PatField { hir_id , ident , pat , is_shorthand : _ , span : _ } = field ; try_visit ! (visitor . visit_id (* hir_id)) ; try_visit ! (visitor . visit_ident (* ident)) ; visitor . visit_pat (* pat) }
};
}
