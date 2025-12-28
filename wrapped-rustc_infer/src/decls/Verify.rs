macro_rules! deps {
    () => {
        SubregionOrigin!();
        VerifyBound!();
        GenericKind!();
    };
}

macro_rules! Verify {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct Verify < 'tcx > { pub kind : GenericKind < 'tcx > , pub origin : SubregionOrigin < 'tcx > , pub region : Region < 'tcx > , pub bound : VerifyBound < 'tcx > , }
    };
}

Verify!();