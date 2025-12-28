macro_rules! deps {
    () => {
        DefUseResult!();
    };
}

macro_rules! DefUseVisitor {
    () => {
        deps!();
        struct DefUseVisitor < 'a , 'tcx > { body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , region_vid : RegionVid , def_use_result : Option < DefUseResult > , }
    };
}

DefUseVisitor!()