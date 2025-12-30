// Generated macro for walk_trait_ref (function)
macro_rules! Depcrate_intravisitwalk_trait_ref {
() => {
// Module: crate::intravisit
// Provides: {"walk_trait_ref"}
// Dependencies: {}
pub fn walk_trait_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , trait_ref : & 'v TraitRef < 'v > ,) -> V :: Result { let TraitRef { hir_ref_id , path } = trait_ref ; try_visit ! (visitor . visit_id (* hir_ref_id)) ; visitor . visit_path (* path , * hir_ref_id) }
};
}
