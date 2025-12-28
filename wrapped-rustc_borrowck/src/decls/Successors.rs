macro_rules! deps {
    () => {
        EdgesFromStatic!();
        ConstraintGraphDirection!();
        EdgesFromGraph!();
    };
}

macro_rules! Successors {
    () => {
        deps!();
        pub (crate) enum Successors < 'a , 'tcx , D : ConstraintGraphDirection > { FromStatic (EdgesFromStatic) , FromGraph (EdgesFromGraph < 'a , 'tcx , D >) , }
    };
}

Successors!();