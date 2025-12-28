macro_rules! check_const_item {
    () => {
        pub (crate) fn check_const_item (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Result < () , ErrorGuaranteed > { enter_wf_checking_ctxt (tcx , def_id , | wfcx | { let ty = tcx . type_of (def_id) . instantiate_identity () ; let ty_span = tcx . ty_span (def_id) ; let ty = wfcx . deeply_normalize (ty_span , Some (WellFormedLoc :: Ty (def_id)) , ty) ; wfcx . register_wf_obligation (ty_span , Some (WellFormedLoc :: Ty (def_id)) , ty . into ()) ; wfcx . register_bound (traits :: ObligationCause :: new (ty_span , wfcx . body_def_id , ObligationCauseCode :: SizedConstOrStatic ,) , wfcx . param_env , ty , tcx . require_lang_item (LangItem :: Sized , ty_span) ,) ; check_where_clauses (wfcx , def_id) ; Ok (()) }) }
    };
}

check_const_item!();