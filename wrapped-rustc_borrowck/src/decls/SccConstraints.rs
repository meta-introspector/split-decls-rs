macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! SccConstraints {
    () => {
        deps!();
        struct SccConstraints < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , nodes_per_scc : IndexVec < ConstraintSccIndex , Vec < RegionVid > > , }
    };
}

SccConstraints!()