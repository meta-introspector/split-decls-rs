macro_rules! deps {
    () => {
        PredicateObligation!();
    };
}

macro_rules! Selection {
    () => {
        deps!();
        pub type Selection < 'tcx > = ImplSource < 'tcx , PredicateObligation < 'tcx > > ;
    };
}

Selection!()