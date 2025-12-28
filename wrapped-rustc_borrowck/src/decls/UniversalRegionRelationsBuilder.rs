macro_rules! deps {
    () => {
        MirTypeckRegionConstraints!();
        UniversalRegions!();
        BorrowckInferCtxt!();
    };
}

macro_rules! UniversalRegionRelationsBuilder {
    () => {
        deps!();
        struct UniversalRegionRelationsBuilder < 'a , 'tcx > { infcx : & 'a BorrowckInferCtxt < 'tcx > , universal_regions : UniversalRegions < 'tcx > , constraints : & 'a mut MirTypeckRegionConstraints < 'tcx > , outlives : TransitiveRelationBuilder < RegionVid > , inverse_outlives : TransitiveRelationBuilder < RegionVid > , region_bound_pairs : RegionBoundPairs < 'tcx > , }
    };
}

UniversalRegionRelationsBuilder!()