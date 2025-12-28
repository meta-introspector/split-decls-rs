macro_rules! PredicateSet {
    () => {
        pub struct PredicateSet < 'tcx > { tcx : TyCtxt < 'tcx > , set : FxHashSet < ty :: Predicate < 'tcx > > , }
    };
}

PredicateSet!();