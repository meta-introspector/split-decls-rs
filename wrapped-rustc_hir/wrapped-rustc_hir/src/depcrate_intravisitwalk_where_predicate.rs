// Generated macro for walk_where_predicate (function)
macro_rules! Depcrate_intravisitwalk_where_predicate {
() => {
// Module: crate::intravisit
// Provides: {"walk_where_predicate"}
// Dependencies: {}
pub fn walk_where_predicate < 'v , V : Visitor < 'v > > (visitor : & mut V , predicate : & 'v WherePredicate < 'v > ,) -> V :: Result { let & WherePredicate { hir_id , kind , span : _ } = predicate ; try_visit ! (visitor . visit_id (hir_id)) ; match * kind { WherePredicateKind :: BoundPredicate (WhereBoundPredicate { ref bounded_ty , bounds , bound_generic_params , origin : _ , }) => { try_visit ! (visitor . visit_ty_unambig (bounded_ty)) ; walk_list ! (visitor , visit_param_bound , bounds) ; walk_list ! (visitor , visit_generic_param , bound_generic_params) ; } WherePredicateKind :: RegionPredicate (WhereRegionPredicate { ref lifetime , bounds , in_where_clause : _ , }) => { try_visit ! (visitor . visit_lifetime (lifetime)) ; walk_list ! (visitor , visit_param_bound , bounds) ; } WherePredicateKind :: EqPredicate (WhereEqPredicate { ref lhs_ty , ref rhs_ty }) => { try_visit ! (visitor . visit_ty_unambig (lhs_ty)) ; try_visit ! (visitor . visit_ty_unambig (rhs_ty)) ; } } V :: Result :: output () }
};
}
