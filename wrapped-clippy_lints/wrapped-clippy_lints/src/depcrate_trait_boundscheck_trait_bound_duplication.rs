// Generated macro for check_trait_bound_duplication (function)
macro_rules! Depcrate_trait_boundscheck_trait_bound_duplication {
() => {
// Module: crate::trait_bounds
// Provides: {"check_trait_bound_duplication"}
// Dependencies: {}
fn check_trait_bound_duplication < 'tcx > (cx : & LateContext < 'tcx > , generics : & '_ Generics < 'tcx >) { if generics . span . from_expansion () { return ; } # [expect (clippy :: mutable_key_type)] let where_predicates = generics . predicates . iter () . filter_map (| pred | { if pred . kind . in_where_clause () && let WherePredicateKind :: BoundPredicate (bound_predicate) = pred . kind && let TyKind :: Path (QPath :: Resolved (_ , path)) = bound_predicate . bounded_ty . kind { return Some (rollup_traits (cx , bound_predicate . bounds , "these where clauses contain repeated elements" ,) . into_iter () . map (| (trait_ref , _) | (path . res , trait_ref)) ,) ; } None }) . flatten () . collect :: < FxHashSet < _ > > () ; for predicate in generics . predicates . iter () . filter (| pred | ! pred . kind . in_where_clause ()) { if let WherePredicateKind :: BoundPredicate (bound_predicate) = predicate . kind && bound_predicate . origin != PredicateOrigin :: ImplTrait && ! predicate . span . from_expansion () && let TyKind :: Path (QPath :: Resolved (_ , path)) = bound_predicate . bounded_ty . kind { let traits = rollup_traits (cx , bound_predicate . bounds , "these bounds contain repeated elements") ; for (trait_ref , span) in traits { let key = (path . res , trait_ref) ; if where_predicates . contains (& key) { span_lint_and_help (cx , TRAIT_DUPLICATION_IN_BOUNDS , span , "this trait bound is already specified in the where clause" , None , "consider removing this trait bound" ,) ; } } } } }
};
}
