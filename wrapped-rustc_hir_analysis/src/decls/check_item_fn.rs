macro_rules! check_item_fn {
    () => {
        fn check_item_fn (tcx : TyCtxt < '_ > , def_id : LocalDefId , decl : & hir :: FnDecl < '_ > ,) -> Result < () , ErrorGuaranteed > { enter_wf_checking_ctxt (tcx , def_id , | wfcx | { let sig = tcx . fn_sig (def_id) . instantiate_identity () ; check_fn_or_method (wfcx , sig , decl , def_id) ; Ok (()) }) }
    };
}

check_item_fn!()