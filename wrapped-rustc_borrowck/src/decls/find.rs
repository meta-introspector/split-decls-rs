macro_rules! deps {
    () => {
        UseFinder!();
        Cause!();
        RegionInferenceContext!();
    };
}

macro_rules! find {
    () => {
        deps!();
        pub (crate) fn find < 'tcx > (body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , tcx : TyCtxt < 'tcx > , region_vid : RegionVid , start_point : Location ,) -> Option < Cause > { let mut uf = UseFinder { body , regioncx , tcx , region_vid , start_point } ; uf . find () }
    };
}

find!()