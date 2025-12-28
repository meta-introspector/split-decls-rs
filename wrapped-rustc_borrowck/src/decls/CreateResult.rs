macro_rules! deps {
    () => {
        NormalizedInputsAndOutput!();
        UniversalRegionRelations!();
    };
}

macro_rules! CreateResult {
    () => {
        deps!();
        pub (crate) struct CreateResult < 'tcx > { pub (crate) universal_region_relations : Frozen < UniversalRegionRelations < 'tcx > > , pub (crate) region_bound_pairs : Frozen < RegionBoundPairs < 'tcx > > , pub (crate) known_type_outlives_obligations : Frozen < Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > > , pub (crate) normalized_inputs_and_output : NormalizedInputsAndOutput < 'tcx > , }
    };
}

CreateResult!()