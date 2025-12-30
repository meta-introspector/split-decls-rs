// Generated macro for get_input_traits_and_projections (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedget_input_traits_and_projections {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"get_input_traits_and_projections"}
// Dependencies: {}
# [doc = " Returns the `TraitPredicate`s and `ProjectionPredicate`s for a function's input type."] fn get_input_traits_and_projections < 'tcx > (cx : & LateContext < 'tcx > , callee_def_id : DefId , input : Ty < 'tcx > ,) -> (Vec < TraitPredicate < 'tcx > > , Vec < ProjectionPredicate < 'tcx > >) { let mut trait_predicates = Vec :: new () ; let mut projection_predicates = Vec :: new () ; for predicate in cx . tcx . param_env (callee_def_id) . caller_bounds () { match predicate . kind () . skip_binder () { ClauseKind :: Trait (trait_predicate) => { if trait_predicate . trait_ref . self_ty () == input { trait_predicates . push (trait_predicate) ; } } , ClauseKind :: Projection (projection_predicate) => { if projection_predicate . projection_term . self_ty () == input { projection_predicates . push (projection_predicate) ; } } , _ => { } , } } (trait_predicates , projection_predicates) }
};
}
