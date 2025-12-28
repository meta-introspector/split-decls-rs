macro_rules! deps {
    () => {
        UniversalRegions!();
        PoloniusLivenessContext!();
        LivenessValues!();
    };
}

macro_rules! LiveVariablesVisitor {
    () => {
        deps!();
        # [doc = " Visitor looking for regions that should be live within rvalues or calls."] struct LiveVariablesVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , liveness_constraints : & 'a mut LivenessValues , universal_regions : & 'a UniversalRegions < 'tcx > , polonius_liveness : & 'a mut Option < PoloniusLivenessContext > , }
    };
}

LiveVariablesVisitor!();