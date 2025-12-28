macro_rules! deps {
    () => {
        TermsContext!();
        VarianceTermPtr!();
        Constraint!();
    };
}

macro_rules! ConstraintContext {
    () => {
        deps!();
        pub (crate) struct ConstraintContext < 'a , 'tcx > { pub terms_cx : TermsContext < 'a , 'tcx > , covariant : VarianceTermPtr < 'a > , contravariant : VarianceTermPtr < 'a > , invariant : VarianceTermPtr < 'a > , bivariant : VarianceTermPtr < 'a > , pub constraints : Vec < Constraint < 'a > > , }
    };
}

ConstraintContext!()