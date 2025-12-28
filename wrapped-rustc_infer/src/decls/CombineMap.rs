macro_rules! deps {
    () => {
        TwoRegions!();
    };
}

macro_rules! CombineMap {
    () => {
        deps!();
        type CombineMap < 'tcx > = FxHashMap < TwoRegions < 'tcx > , RegionVid > ;
    };
}

CombineMap!();