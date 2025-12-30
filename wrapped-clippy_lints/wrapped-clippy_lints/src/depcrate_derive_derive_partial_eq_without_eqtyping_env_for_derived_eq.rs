// Generated macro for typing_env_for_derived_eq (function)
macro_rules! Depcrate_derive_derive_partial_eq_without_eqtyping_env_for_derived_eq {
() => {
// Module: crate::derive::derive_partial_eq_without_eq
// Provides: {"typing_env_for_derived_eq"}
// Dependencies: {}
# [doc = " Creates the `ParamEnv` used for the given type's derived `Eq` impl."] fn typing_env_for_derived_eq (tcx : TyCtxt < '_ > , did : DefId , eq_trait_id : DefId) -> ty :: TypingEnv < '_ > { let mut params = tcx . generics_of (did) . own_params . iter () . map (| p | (p , matches ! (p . kind , GenericParamDefKind :: Type { .. }))) . collect :: < Vec < _ > > () ; let ty_predicates = tcx . predicates_of (did) . predicates ; for (p , _) in ty_predicates { if let ClauseKind :: Trait (p) = p . kind () . skip_binder () && p . trait_ref . def_id == eq_trait_id && let ty :: Param (self_ty) = p . trait_ref . self_ty () . kind () { params [self_ty . index as usize] . 1 = false ; } } let param_env = ParamEnv :: new (tcx . mk_clauses_from_iter (ty_predicates . iter () . map (| & (p , _) | p) . chain (params . iter () . filter (| & & (_ , needs_eq) | needs_eq) . map (| & (param , _) | { ClauseKind :: Trait (TraitPredicate { trait_ref : ty :: TraitRef :: new (tcx , eq_trait_id , [tcx . mk_param_from_def (param)]) , polarity : ty :: PredicatePolarity :: Positive , }) . upcast (tcx) }) ,))) ; ty :: TypingEnv { typing_mode : ty :: TypingMode :: non_body_analysis () , param_env , } }
};
}
