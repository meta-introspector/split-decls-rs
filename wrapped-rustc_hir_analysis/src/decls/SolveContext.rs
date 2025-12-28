macro_rules! deps {
    () => {
        Constraint!();
        TermsContext!();
    };
}

macro_rules! SolveContext {
    () => {
        deps!();
        struct SolveContext < 'a , 'tcx > { terms_cx : TermsContext < 'a , 'tcx > , constraints : Vec < Constraint < 'a > > , solutions : Vec < ty :: Variance > , }
    };
}

SolveContext!();