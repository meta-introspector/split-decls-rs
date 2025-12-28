macro_rules! deps {
    () => {
        MatchAgainstHigherRankedOutlives!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'tcx > MatchAgainstHigherRankedOutlives < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> MatchAgainstHigherRankedOutlives < 'tcx > { MatchAgainstHigherRankedOutlives { tcx , pattern_depth : ty :: INNERMOST , map : FxHashMap :: default () , } } }
    };
}

impl_100!();