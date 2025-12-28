macro_rules! deps {
    () => {
        WfCheckingCtxt!();
    };
}

macro_rules! check_associated_type_bounds {
    () => {
        deps!();
        # [doc = " Checks all associated type defaults of trait `trait_def_id`."] # [doc = ""] # [doc = " Assuming the defaults are used, check that all predicates (bounds on the"] # [doc = " assoc type and where clauses on the trait) hold."] fn check_associated_type_bounds (wfcx : & WfCheckingCtxt < '_ , '_ > , item : ty :: AssocItem , span : Span) { let bounds = wfcx . tcx () . explicit_item_bounds (item . def_id) ; debug ! ("check_associated_type_bounds: bounds={:?}" , bounds) ; let wf_obligations = bounds . iter_identity_copied () . flat_map (| (bound , bound_span) | { let normalized_bound = wfcx . normalize (span , None , bound) ; traits :: wf :: clause_obligations (wfcx . infcx , wfcx . param_env , wfcx . body_def_id , normalized_bound , bound_span ,) }) ; wfcx . register_obligations (wf_obligations) ; }
    };
}

check_associated_type_bounds!();