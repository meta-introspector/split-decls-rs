macro_rules! deps {
    () => {
        Normalized!();
    };
}

macro_rules! NormalizedTerm {
    () => {
        deps!();
        pub type NormalizedTerm < 'tcx > = Normalized < 'tcx , ty :: Term < 'tcx > > ;
    };
}

NormalizedTerm!()