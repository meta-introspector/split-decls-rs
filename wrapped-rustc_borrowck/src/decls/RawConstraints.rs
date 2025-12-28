macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! RawConstraints {
    () => {
        deps!();
        struct RawConstraints < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , }
    };
}

RawConstraints!();