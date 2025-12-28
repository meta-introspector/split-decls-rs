macro_rules! deps {
    () => {
        RequiredPredicates!();
    };
}

macro_rules! ExplicitPredicatesMap {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ExplicitPredicatesMap < 'tcx > { map : FxIndexMap < DefId , ty :: EarlyBinder < 'tcx , RequiredPredicates < 'tcx > > > , }
    };
}

ExplicitPredicatesMap!();