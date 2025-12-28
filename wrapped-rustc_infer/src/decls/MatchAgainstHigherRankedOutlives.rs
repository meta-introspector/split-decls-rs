macro_rules! MatchAgainstHigherRankedOutlives {
    () => {
        struct MatchAgainstHigherRankedOutlives < 'tcx > { tcx : TyCtxt < 'tcx > , pattern_depth : ty :: DebruijnIndex , map : FxHashMap < ty :: BoundRegion , ty :: Region < 'tcx > > , }
    };
}

MatchAgainstHigherRankedOutlives!();