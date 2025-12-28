macro_rules! deps {
    () => {
        WfCheckingCtxt!();
    };
}

macro_rules! check_fn_or_method {
    () => {
        deps!();
        # [instrument (level = "debug" , skip (wfcx , hir_decl))] fn check_fn_or_method < 'tcx > (wfcx : & WfCheckingCtxt < '_ , 'tcx > , sig : ty :: PolyFnSig < 'tcx > , hir_decl : & hir :: FnDecl < '_ > , def_id : LocalDefId ,) { let tcx = wfcx . tcx () ; let mut sig = tcx . liberate_late_bound_regions (def_id . to_def_id () , sig) ; let arg_span = | idx | hir_decl . inputs . get (idx) . map_or (hir_decl . output . span () , | arg : & hir :: Ty < '_ > | arg . span) ; sig . inputs_and_output = tcx . mk_type_list_from_iter (sig . inputs_and_output . iter () . enumerate () . map (| (idx , ty) | { wfcx . deeply_normalize (arg_span (idx) , Some (WellFormedLoc :: Param { function : def_id , param_idx : idx , }) , ty ,) })) ; for (idx , ty) in sig . inputs_and_output . iter () . enumerate () { wfcx . register_wf_obligation (arg_span (idx) , Some (WellFormedLoc :: Param { function : def_id , param_idx : idx }) , ty . into () ,) ; } check_where_clauses (wfcx , def_id) ; if sig . abi == ExternAbi :: RustCall { let span = tcx . def_span (def_id) ; let has_implicit_self = hir_decl . implicit_self != hir :: ImplicitSelfKind :: None ; let mut inputs = sig . inputs () . iter () . skip (if has_implicit_self { 1 } else { 0 }) ; if let Some (ty) = inputs . next () { wfcx . register_bound (ObligationCause :: new (span , wfcx . body_def_id , ObligationCauseCode :: RustCall) , wfcx . param_env , * ty , tcx . require_lang_item (hir :: LangItem :: Tuple , span) ,) ; wfcx . register_bound (ObligationCause :: new (span , wfcx . body_def_id , ObligationCauseCode :: RustCall) , wfcx . param_env , * ty , tcx . require_lang_item (hir :: LangItem :: Sized , span) ,) ; } else { tcx . dcx () . span_err (hir_decl . inputs . last () . map_or (span , | input | input . span) , "functions with the \"rust-call\" ABI must take a single non-self tuple argument" ,) ; } if inputs . next () . is_some () { tcx . dcx () . span_err (hir_decl . inputs . last () . map_or (span , | input | input . span) , "functions with the \"rust-call\" ABI must take a single non-self tuple argument" ,) ; } } check_sized_if_body (wfcx , def_id , sig . output () , match hir_decl . output { hir :: FnRetTy :: Return (ty) => Some (ty . span) , hir :: FnRetTy :: DefaultReturn (_) => None , } , ObligationCauseCode :: SizedReturnType ,) ; }
    };
}

check_fn_or_method!();