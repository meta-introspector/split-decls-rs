macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! PolyTraitObligation {
    () => {
        deps!();
        pub type PolyTraitObligation < 'tcx > = Obligation < 'tcx , ty :: PolyTraitPredicate < 'tcx > > ;
    };
}

PolyTraitObligation!();