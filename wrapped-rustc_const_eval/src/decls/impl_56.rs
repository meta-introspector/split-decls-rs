macro_rules! deps {
    () => {
        HasMutInterior!();
        Qualif!();
        ConstCx!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Qualif for HasMutInterior { const ANALYSIS_NAME : & 'static str = "flow_has_mut_interior" ; fn in_qualifs (qualifs : & ConstQualifs) -> bool { qualifs . has_mut_interior } fn in_any_value_of_ty < 'tcx > (cx : & ConstCx < '_ , 'tcx > , ty : Ty < 'tcx >) -> bool { if ty . is_trivially_freeze () { return false ; } if ty . ty_adt_def () . is_some_and (| adt | adt . is_unsafe_cell ()) { return true ; } let freeze_def_id = cx . tcx . require_lang_item (LangItem :: Freeze , cx . body . span) ; let typing_env = ty :: TypingEnv { typing_mode : ty :: TypingMode :: analysis_in_body (cx . tcx , cx . body . source . def_id () . expect_local () ,) , param_env : cx . typing_env . param_env , } ; let (infcx , param_env) = cx . tcx . infer_ctxt () . build_with_typing_env (typing_env) ; let ocx = ObligationCtxt :: new (& infcx) ; let obligation = Obligation :: new (cx . tcx , ObligationCause :: dummy_with_span (cx . body . span) , param_env , ty :: TraitRef :: new (cx . tcx , freeze_def_id , [ty :: GenericArg :: from (ty)]) ,) ; ocx . register_obligation (obligation) ; let errors = ocx . select_all_or_error () ; ! errors . is_empty () } fn is_structural_in_adt_value < 'tcx > (_cx : & ConstCx < '_ , 'tcx > , adt : AdtDef < 'tcx >) -> bool { ! adt . is_unsafe_cell () } }
    };
}

impl_56!();