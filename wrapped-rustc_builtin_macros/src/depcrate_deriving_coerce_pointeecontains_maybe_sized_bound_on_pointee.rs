// Generated macro for contains_maybe_sized_bound_on_pointee (function)
macro_rules! Depcrate_deriving_coerce_pointeecontains_maybe_sized_bound_on_pointee {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"contains_maybe_sized_bound_on_pointee"}
// Dependencies: {}
fn contains_maybe_sized_bound_on_pointee (predicates : & [WherePredicate] , pointee : Symbol) -> bool { for bound in predicates { if let ast :: WherePredicateKind :: BoundPredicate (bound) = & bound . kind && bound . bounded_ty . kind . is_simple_path () . is_some_and (| name | name == pointee) { for bound in & bound . bounds { if is_maybe_sized_bound (bound) { return true ; } } } } false }
};
}
