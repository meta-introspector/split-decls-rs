macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! PredicateObligation {
    () => {
        deps!();
        pub type PredicateObligation < 'tcx > = Obligation < 'tcx , ty :: Predicate < 'tcx > > ;
    };
}

PredicateObligation!()