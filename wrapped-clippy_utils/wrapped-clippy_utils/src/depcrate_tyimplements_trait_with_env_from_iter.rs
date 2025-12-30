// Generated macro for implements_trait_with_env_from_iter (function)
macro_rules! Depcrate_tyimplements_trait_with_env_from_iter {
() => {
// Module: crate::ty
// Provides: {"implements_trait_with_env_from_iter"}
// Dependencies: {}
# [doc = " Same as `implements_trait_from_env` but takes the arguments as an iterator."] pub fn implements_trait_with_env_from_iter < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > , trait_id : DefId , callee_id : Option < DefId > , args : impl IntoIterator < Item = impl Into < Option < GenericArg < 'tcx > > > > ,) -> bool { assert ! (! ty . has_infer ()) ; if let Some (callee_id) = callee_id { let _ = tcx . hir_body_owner_kind (callee_id) ; } let ty = tcx . erase_and_anonymize_regions (ty) ; if ty . has_escaping_bound_vars () { return false ; } let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let args = args . into_iter () . map (| arg | arg . into () . unwrap_or_else (| | infcx . next_ty_var (DUMMY_SP) . into ())) . collect :: < Vec < _ > > () ; let trait_ref = TraitRef :: new (tcx , trait_id , [GenericArg :: from (ty)] . into_iter () . chain (args)) ; debug_assert_matches ! (tcx . def_kind (trait_id) , DefKind :: Trait | DefKind :: TraitAlias , "`DefId` must belong to a trait or trait alias") ; # [cfg (debug_assertions)] assert_generic_args_match (tcx , trait_id , trait_ref . args) ; let obligation = Obligation { cause : ObligationCause :: dummy () , param_env , recursion_depth : 0 , predicate : trait_ref . upcast (tcx) , } ; infcx . evaluate_obligation (& obligation) . is_ok_and (EvaluationResult :: must_apply_modulo_regions) }
};
}
