// Generated macro for collect_supertrait_bounds (function)
macro_rules! Depcrate_implied_bounds_in_implscollect_supertrait_bounds {
() => {
// Module: crate::implied_bounds_in_impls
// Provides: {"collect_supertrait_bounds"}
// Dependencies: {}
# [doc = " Given an `impl Trait` type, gets all the supertraits from each bound (\"implied bounds\")."] # [doc = ""] # [doc = " For `impl Deref + DerefMut + Eq` this returns `[Deref, PartialEq]`."] # [doc = " The `Deref` comes from `DerefMut` because `trait DerefMut: Deref {}`, and `PartialEq` comes from"] # [doc = " `Eq`."] fn collect_supertrait_bounds < 'tcx > (cx : & LateContext < 'tcx > , bounds : GenericBounds < 'tcx >) -> Vec < ImplTraitBound < 'tcx > > { bounds . iter () . filter_map (| bound | { if let GenericBound :: Trait (poly_trait) = bound && let TraitBoundModifiers :: NONE = poly_trait . modifiers && let [.. , path] = poly_trait . trait_ref . path . segments && poly_trait . bound_generic_params . is_empty () && let Some (trait_def_id) = path . res . opt_def_id () && let predicates = cx . tcx . explicit_super_predicates_of (trait_def_id) . skip_binder () && ! predicates . is_empty () { Some (ImplTraitBound { span : bound . span () , predicates , trait_def_id , args : path . args . map_or ([] . as_slice () , | p | p . args) , constraints : path . args . map_or ([] . as_slice () , | p | p . constraints) , }) } else { None } }) . collect () }
};
}
