macro_rules! deps {
    () => {
        MirTypeckRegionConstraints!();
        BorrowckInferCtxt!();
        CreateResult!();
        UniversalRegionRelationsBuilder!();
        UniversalRegions!();
    };
}

macro_rules! create {
    () => {
        deps!();
        pub (crate) fn create < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_regions : UniversalRegions < 'tcx > , constraints : & mut MirTypeckRegionConstraints < 'tcx > ,) -> CreateResult < 'tcx > { UniversalRegionRelationsBuilder { infcx , constraints , universal_regions , region_bound_pairs : Default :: default () , outlives : Default :: default () , inverse_outlives : Default :: default () , } . create () }
    };
}

create!();