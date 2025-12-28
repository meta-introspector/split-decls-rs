macro_rules! crate_variances {
    () => {
        pub (super) fn crate_variances (tcx : TyCtxt < '_ > , () : ()) -> CrateVariancesMap < '_ > { let arena = DroplessArena :: default () ; let terms_cx = terms :: determine_parameters_to_be_inferred (tcx , & arena) ; let constraints_cx = constraints :: add_constraints_from_crate (terms_cx) ; solve :: solve_constraints (constraints_cx) }
    };
}

crate_variances!();