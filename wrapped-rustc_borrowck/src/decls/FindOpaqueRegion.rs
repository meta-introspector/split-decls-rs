macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! FindOpaqueRegion {
    () => {
        deps!();
        # [doc = " This visitor contains the bulk of the logic for this lint."] struct FindOpaqueRegion < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , borrow_region : ty :: RegionVid , }
    };
}

FindOpaqueRegion!()