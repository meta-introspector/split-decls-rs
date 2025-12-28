macro_rules! deps {
    () => {
        UniversalRegions!();
        LivenessValues!();
        PoloniusLivenessContext!();
        LiveVariablesVisitor!();
    };
}

macro_rules! record_regular_live_regions {
    () => {
        deps!();
        # [doc = " Some variables are \"regular live\" at `location` -- i.e., they may be used later. This means that"] # [doc = " all regions appearing in their type must be live at `location`."] fn record_regular_live_regions < 'tcx > (tcx : TyCtxt < 'tcx > , liveness_constraints : & mut LivenessValues , universal_regions : & UniversalRegions < 'tcx > , polonius_liveness : & mut Option < PoloniusLivenessContext > , body : & Body < 'tcx > ,) { let mut visitor = LiveVariablesVisitor { tcx , liveness_constraints , universal_regions , polonius_liveness } ; for (bb , data) in body . basic_blocks . iter_enumerated () { visitor . visit_basic_block_data (bb , data) ; } }
    };
}

record_regular_live_regions!();