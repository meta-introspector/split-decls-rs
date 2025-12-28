macro_rules! deps {
    () => {
        CguReuseTracker!();
    };
}

macro_rules! AssertModuleSource {
    () => {
        deps!();
        struct AssertModuleSource < 'tcx > { tcx : TyCtxt < 'tcx > , available_cgus : UnordSet < Symbol > , cgu_reuse_tracker : CguReuseTracker , }
    };
}

AssertModuleSource!()