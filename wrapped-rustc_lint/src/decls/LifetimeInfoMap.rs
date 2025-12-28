macro_rules! deps {
    () => {
        Info!();
    };
}

macro_rules! LifetimeInfoMap {
    () => {
        deps!();
        type LifetimeInfoMap < 'tcx > = FxIndexMap < & 'tcx hir :: LifetimeKind , Vec < Info < 'tcx > > > ;
    };
}

LifetimeInfoMap!();