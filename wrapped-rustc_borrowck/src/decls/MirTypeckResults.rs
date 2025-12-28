macro_rules! deps {
    () => {
        MirTypeckRegionConstraints!();
        PoloniusContext!();
        DeferredClosureRequirements!();
        UniversalRegionRelations!();
    };
}

macro_rules! MirTypeckResults {
    () => {
        deps!();
        # [doc = " Holder struct for passing results from MIR typeck to the rest of the non-lexical regions"] # [doc = " inference computation."] pub (crate) struct MirTypeckResults < 'tcx > { pub (crate) constraints : MirTypeckRegionConstraints < 'tcx > , pub (crate) universal_region_relations : Frozen < UniversalRegionRelations < 'tcx > > , pub (crate) region_bound_pairs : Frozen < RegionBoundPairs < 'tcx > > , pub (crate) known_type_outlives_obligations : Frozen < Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > > , pub (crate) deferred_closure_requirements : DeferredClosureRequirements < 'tcx > , pub (crate) polonius_context : Option < PoloniusContext > , }
    };
}

MirTypeckResults!()