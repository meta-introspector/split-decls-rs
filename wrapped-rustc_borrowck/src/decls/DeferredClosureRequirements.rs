macro_rules! DeferredClosureRequirements {
    () => {
        type DeferredClosureRequirements < 'tcx > = Vec < (LocalDefId , ty :: GenericArgsRef < 'tcx > , Locations) > ;
    };
}

DeferredClosureRequirements!()