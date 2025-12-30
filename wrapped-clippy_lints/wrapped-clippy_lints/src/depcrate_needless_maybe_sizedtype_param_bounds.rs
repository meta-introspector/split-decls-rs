// Generated macro for type_param_bounds (function)
macro_rules! Depcrate_needless_maybe_sizedtype_param_bounds {
() => {
// Module: crate::needless_maybe_sized
// Provides: {"type_param_bounds"}
// Dependencies: {}
# [doc = " Finds all of the [`Bound`]s that refer to a type parameter and are not from a macro expansion"] fn type_param_bounds < 'tcx > (generics : & 'tcx Generics < 'tcx >) -> impl Iterator < Item = Bound < 'tcx > > { generics . predicates . iter () . enumerate () . filter_map (| (predicate_pos , predicate) | { let WherePredicateKind :: BoundPredicate (bound_predicate) = & predicate . kind else { return None ; } ; let (param , ident) = bound_predicate . bounded_ty . as_generic_param () ? ; Some (bound_predicate . bounds . iter () . enumerate () . filter_map (move | (bound_pos , bound) | match bound { GenericBound :: Trait (trait_bound) => Some (Bound { param , ident , trait_bound , predicate_pos , bound_pos , }) , GenericBound :: Outlives (_) | GenericBound :: Use (..) => None , }) . filter (| bound | ! bound . trait_bound . span . from_expansion ()) ,) }) . flatten () }
};
}
