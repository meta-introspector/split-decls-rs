// Generated macro for check (function)
macro_rules! Depcrate_implied_bounds_in_implscheck {
() => {
// Module: crate::implied_bounds_in_impls
// Provides: {"check"}
// Dependencies: {}
fn check < 'tcx > (cx : & LateContext < 'tcx > , bounds : GenericBounds < 'tcx >) { if bounds . len () == 1 { return ; } let supertraits = collect_supertrait_bounds (cx , bounds) ; for (index , bound) in bounds . iter () . enumerate () { if let GenericBound :: Trait (poly_trait) = bound && let TraitBoundModifiers :: NONE = poly_trait . modifiers && let [.. , path] = poly_trait . trait_ref . path . segments && let implied_args = path . args . map_or ([] . as_slice () , | a | a . args) && let implied_constraints = path . args . map_or ([] . as_slice () , | a | a . constraints) && let Some (def_id) = poly_trait . trait_ref . path . res . opt_def_id () && let Some (bound) = find_bound_in_supertraits (cx , def_id , implied_args , & supertraits) && let assocs = cx . tcx . associated_items (bound . trait_def_id) && ! implied_constraints . iter () . any (| constraint | { assocs . filter_by_name_unhygienic (constraint . ident . name) . next () . is_some_and (AssocItem :: is_type) }) { emit_lint (cx , poly_trait , bounds , index , implied_constraints , bound) ; } } }
};
}
