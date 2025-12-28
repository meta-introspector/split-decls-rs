macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! suggest_question_mark {
    () => {
        deps!();
        fn suggest_question_mark < 'tcx > (cx : & LateContext < 'tcx > , adt : ty :: AdtDef < 'tcx > , args : ty :: GenericArgsRef < 'tcx > , span : Span ,) -> bool { let Some (body_id) = cx . enclosing_body else { return false } ; let Some (into_iterator_did) = cx . tcx . get_diagnostic_item (sym :: IntoIterator) else { return false ; } ; if ! cx . tcx . is_diagnostic_item (sym :: Result , adt . did ()) { return false ; } { let ty = cx . typeck_results () . expr_ty (cx . tcx . hir_body (body_id) . value) ; let ty :: Adt (ret_adt , ..) = ty . kind () else { return false } ; if ! cx . tcx . is_diagnostic_item (sym :: Result , ret_adt . did ()) { return false ; } } let ty = args . type_at (0) ; let (infcx , param_env) = cx . tcx . infer_ctxt () . build_with_typing_env (cx . typing_env ()) ; let ocx = ObligationCtxt :: new (& infcx) ; let body_def_id = cx . tcx . hir_body_owner_def_id (body_id) ; let cause = ObligationCause :: new (span , body_def_id , rustc_infer :: traits :: ObligationCauseCode :: Misc) ; ocx . register_bound (cause , param_env , infcx . tcx . erase_and_anonymize_regions (ty) , into_iterator_did ,) ; ocx . select_all_or_error () . is_empty () }
    };
}

suggest_question_mark!();