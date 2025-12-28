macro_rules! check_potentially_region_dependent_goals {
    () => {
        pub (super) fn check_potentially_region_dependent_goals < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { if ! tcx . next_trait_solver_globally () { return Ok (()) ; } let typeck_results = tcx . typeck (def_id) ; let param_env = tcx . param_env (def_id) ; let typing_mode = TypingMode :: borrowck (tcx , def_id) ; let infcx = tcx . infer_ctxt () . ignoring_regions () . build (typing_mode) ; let ocx = ObligationCtxt :: new_with_diagnostics (& infcx) ; for (predicate , cause) in & typeck_results . potentially_region_dependent_goals { let predicate = fold_regions (tcx , * predicate , | _ , _ | { infcx . next_region_var (RegionVariableOrigin :: Misc (cause . span)) }) ; ocx . register_obligation (Obligation :: new (tcx , cause . clone () , param_env , predicate)) ; } let errors = ocx . select_all_or_error () ; debug ! (? errors) ; if errors . is_empty () { Ok (()) } else { Err (infcx . err_ctxt () . report_fulfillment_errors (errors)) } }
    };
}

check_potentially_region_dependent_goals!();