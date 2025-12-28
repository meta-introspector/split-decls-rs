macro_rules! deps {
    () => {
        GenericKind!();
    };
}

macro_rules! RegionBoundPairs {
    () => {
        deps!();
        # [doc = " \"Region-bound pairs\" tracks outlives relations that are known to"] # [doc = " be true, either because of explicit where-clauses like `T: 'a` or"] # [doc = " because of implied bounds."] pub type RegionBoundPairs < 'tcx > = FxIndexSet < ty :: OutlivesPredicate < 'tcx , GenericKind < 'tcx > > > ;
    };
}

RegionBoundPairs!()