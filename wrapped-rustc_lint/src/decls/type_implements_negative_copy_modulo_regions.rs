macro_rules! type_implements_negative_copy_modulo_regions {
    () => {
        # [doc = " Check whether a `ty` has a negative `Copy` implementation, ignoring outlives constraints."] fn type_implements_negative_copy_modulo_regions < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> bool { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let trait_ref = ty :: TraitRef :: new (tcx , tcx . require_lang_item (hir :: LangItem :: Copy , DUMMY_SP) , [ty]) ; let pred = ty :: TraitPredicate { trait_ref , polarity : ty :: PredicatePolarity :: Negative } ; let obligation = traits :: Obligation { cause : traits :: ObligationCause :: dummy () , param_env , recursion_depth : 0 , predicate : pred . upcast (tcx) , } ; infcx . predicate_must_hold_modulo_regions (& obligation) }
    };
}

type_implements_negative_copy_modulo_regions!();