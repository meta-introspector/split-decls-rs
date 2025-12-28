macro_rules! deps {
    () => {
        ClosureRegionRequirements!();
        RegionErrors!();
        PoloniusDiagnosticsContext!();
        PoloniusOutput!();
        RegionInferenceContext!();
        PoloniusFacts!();
    };
}

macro_rules! NllOutput {
    () => {
        deps!();
        # [doc = " The output of `nll::compute_regions`. This includes the computed `RegionInferenceContext`, any"] # [doc = " closure requirements to propagate, and any generated errors."] pub (crate) struct NllOutput < 'tcx > { pub regioncx : RegionInferenceContext < 'tcx > , pub polonius_input : Option < Box < PoloniusFacts > > , pub polonius_output : Option < Box < PoloniusOutput > > , pub opt_closure_req : Option < ClosureRegionRequirements < 'tcx > > , pub nll_errors : RegionErrors < 'tcx > , # [doc = " When using `-Zpolonius=next`: the data used to compute errors and diagnostics, e.g."] # [doc = " localized typeck and liveness constraints."] pub polonius_diagnostics : Option < PoloniusDiagnosticsContext > , }
    };
}

NllOutput!();