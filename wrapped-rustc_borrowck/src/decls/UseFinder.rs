macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! UseFinder {
    () => {
        deps!();
        struct UseFinder < 'a , 'tcx > { body : & 'a Body < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , tcx : TyCtxt < 'tcx > , region_vid : RegionVid , start_point : Location , }
    };
}

UseFinder!();