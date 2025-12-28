macro_rules! check_well_formed {
    () => {
        pub (super) fn check_well_formed (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { let mut res = crate :: check :: check :: check_item_type (tcx , def_id) ; for param in & tcx . generics_of (def_id) . own_params { res = res . and (check_param_wf (tcx , param)) ; } res }
    };
}

check_well_formed!();