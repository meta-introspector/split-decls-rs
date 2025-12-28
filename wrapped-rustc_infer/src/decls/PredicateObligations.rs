macro_rules! deps {
    () => {
        PredicateObligation!();
    };
}

macro_rules! PredicateObligations {
    () => {
        deps!();
        pub type PredicateObligations < 'tcx > = ThinVec < PredicateObligation < 'tcx > > ;
    };
}

PredicateObligations!();