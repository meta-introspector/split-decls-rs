// Generated macro for walk_poly_trait_ref (function)
macro_rules! Depcrate_intravisitwalk_poly_trait_ref {
() => {
// Module: crate::intravisit
// Provides: {"walk_poly_trait_ref"}
// Dependencies: {}
pub fn walk_poly_trait_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , trait_ref : & 'v PolyTraitRef < 'v > ,) -> V :: Result { let PolyTraitRef { bound_generic_params , modifiers : _ , trait_ref , span : _ } = trait_ref ; walk_list ! (visitor , visit_generic_param , * bound_generic_params) ; visitor . visit_trait_ref (trait_ref) }
};
}
