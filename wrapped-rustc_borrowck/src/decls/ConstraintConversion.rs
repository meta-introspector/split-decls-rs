macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
        MirTypeckRegionConstraints!();
        UniversalRegions!();
        Locations!();
    };
}

macro_rules! ConstraintConversion {
    () => {
        deps!();
        pub (crate) struct ConstraintConversion < 'a , 'tcx > { infcx : & 'a BorrowckInferCtxt < 'tcx > , universal_regions : & 'a UniversalRegions < 'tcx > , # [doc = " Each RBP `GK: 'a` is assumed to be true. These encode"] # [doc = " relationships like `T: 'a` that are added via implicit bounds"] # [doc = " or the `param_env`."] # [doc = ""] # [doc = " Each region here is guaranteed to be a key in the `indices`"] # [doc = " map. We use the \"original\" regions (i.e., the keys from the"] # [doc = " map, and not the values) because the code in"] # [doc = " `process_registered_region_obligations` has some special-cased"] # [doc = " logic expecting to see (e.g.) `ReStatic`, and if we supplied"] # [doc = " our special inference variable there, we would mess that up."] region_bound_pairs : & 'a RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & 'a [ty :: PolyTypeOutlivesPredicate < 'tcx >] , locations : Locations , span : Span , category : ConstraintCategory < 'tcx > , from_closure : bool , constraints : & 'a mut MirTypeckRegionConstraints < 'tcx > , }
    };
}

ConstraintConversion!()