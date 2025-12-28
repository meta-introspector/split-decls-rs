macro_rules! deps {
    () => {
        Locations!();
    };
}

macro_rules! DeferredClosureRequirements {
    () => {
        deps!();
        type DeferredClosureRequirements < 'tcx > = Vec < (LocalDefId , ty :: GenericArgsRef < 'tcx > , Locations) > ;
    };
}

DeferredClosureRequirements!();