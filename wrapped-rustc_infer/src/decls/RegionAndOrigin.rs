macro_rules! deps {
    () => {
        SubregionOrigin!();
    };
}

macro_rules! RegionAndOrigin {
    () => {
        deps!();
        struct RegionAndOrigin < 'tcx > { region : Region < 'tcx > , origin : SubregionOrigin < 'tcx > , }
    };
}

RegionAndOrigin!()