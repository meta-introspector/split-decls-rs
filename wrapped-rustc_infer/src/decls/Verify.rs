macro_rules! deps {
    () => {
        VerifyBound!();
        GenericKind!();
        SubregionOrigin!();
    };
}

macro_rules! Verify {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct Verify < 'tcx > { pub kind : GenericKind < 'tcx > , pub origin : SubregionOrigin < 'tcx > , pub region : Region < 'tcx > , pub bound : VerifyBound < 'tcx > , }
    };
}

Verify!()