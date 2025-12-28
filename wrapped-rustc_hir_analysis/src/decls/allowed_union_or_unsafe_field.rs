macro_rules! allowed_union_or_unsafe_field {
    () => {
        fn allowed_union_or_unsafe_field < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , span : Span ,) -> bool { if ty . is_trivially_pure_clone_copy () { return true ; } let def_id = tcx . lang_items () . get (LangItem :: BikeshedGuaranteedNoDrop) . unwrap_or_else (| | tcx . require_lang_item (LangItem :: Copy , span)) ; let Ok (ty) = tcx . try_normalize_erasing_regions (typing_env , ty) else { tcx . dcx () . span_delayed_bug (span , "could not normalize field type") ; return true ; } ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; infcx . predicate_must_hold_modulo_regions (& Obligation :: new (tcx , ObligationCause :: dummy_with_span (span) , param_env , ty :: TraitRef :: new (tcx , def_id , [ty]) ,)) }
    };
}

allowed_union_or_unsafe_field!();