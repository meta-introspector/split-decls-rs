// Generated macro for matches_preds (function)
macro_rules! Depcrate_ptr_ptr_argmatches_preds {
() => {
// Module: crate::ptr::ptr_arg
// Provides: {"matches_preds"}
// Dependencies: {}
fn matches_preds < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , preds : & 'tcx [ty :: PolyExistentialPredicate < 'tcx >] ,) -> bool { let infcx = cx . tcx . infer_ctxt () . build (cx . typing_mode ()) ; preds . iter () . all (| & p | match cx . tcx . instantiate_bound_regions_with_erased (p) { ExistentialPredicate :: Trait (p) => infcx . type_implements_trait (p . def_id , [ty . into ()] . into_iter () . chain (p . args . iter ()) , cx . param_env) . must_apply_modulo_regions () , ExistentialPredicate :: Projection (p) => infcx . predicate_must_hold_modulo_regions (& Obligation :: new (cx . tcx , ObligationCause :: dummy () , cx . param_env , cx . tcx . mk_predicate (Binder :: dummy (PredicateKind :: Clause (ClauseKind :: Projection (p . with_self_ty (cx . tcx , ty) ,)))) ,)) , ExistentialPredicate :: AutoTrait (p) => infcx . type_implements_trait (p , [ty] , cx . param_env) . must_apply_modulo_regions () , }) }
};
}
