macro_rules! InherentOverlapChecker {
    () => {
        struct InherentOverlapChecker < 'tcx > { tcx : TyCtxt < 'tcx > , }
    };
}

InherentOverlapChecker!();