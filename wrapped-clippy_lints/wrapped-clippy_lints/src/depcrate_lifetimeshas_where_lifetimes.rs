// Generated macro for has_where_lifetimes (function)
macro_rules! Depcrate_lifetimeshas_where_lifetimes {
() => {
// Module: crate::lifetimes
// Provides: {"has_where_lifetimes"}
// Dependencies: {}
# [doc = " Are any lifetimes mentioned in the `where` clause? If so, we don't try to"] # [doc = " reason about elision."] fn has_where_lifetimes < 'tcx > (cx : & LateContext < 'tcx > , generics : & 'tcx Generics < '_ >) -> bool { for predicate in generics . predicates { match * predicate . kind { WherePredicateKind :: RegionPredicate (..) => return true , WherePredicateKind :: BoundPredicate (ref pred) => { let mut visitor = RefVisitor :: new (cx) ; walk_unambig_ty (& mut visitor , pred . bounded_ty) ; if ! visitor . all_lts () . is_empty () { return true ; } let allowed_lts = allowed_lts_from (pred . bound_generic_params) ; for bound in pred . bounds { walk_param_bound (& mut visitor , bound) ; } for lt in visitor . all_lts () { if let Some (id) = named_lifetime (& lt) && ! allowed_lts . contains (& id) { return true ; } } } , WherePredicateKind :: EqPredicate (ref pred) => { let mut visitor = RefVisitor :: new (cx) ; walk_unambig_ty (& mut visitor , pred . lhs_ty) ; walk_unambig_ty (& mut visitor , pred . rhs_ty) ; if ! visitor . lts . is_empty () { return true ; } } , } } false }
};
}
