macro_rules! deps {
    () => {
        OutlivesConstraintSet!();
        ConstraintGraphDirection!();
        ConstraintGraph!();
    };
}

macro_rules! EdgesFromGraph {
    () => {
        deps!();
        pub (crate) struct EdgesFromGraph < 'a , 'tcx , D : ConstraintGraphDirection > { graph : & 'a ConstraintGraph < D > , constraints : & 'a OutlivesConstraintSet < 'tcx > , pointer : Option < OutlivesConstraintIndex > , }
    };
}

EdgesFromGraph!();