macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! TraitObligation {
    () => {
        deps!();
        pub type TraitObligation < 'tcx > = Obligation < 'tcx , ty :: TraitPredicate < 'tcx > > ;
    };
}

TraitObligation!();