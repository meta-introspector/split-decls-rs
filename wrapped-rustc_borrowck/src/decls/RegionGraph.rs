macro_rules! deps {
    () => {
        ConstraintGraphDirection!();
        OutlivesConstraintSet!();
        ConstraintGraph!();
    };
}

macro_rules! RegionGraph {
    () => {
        deps!();
        # [doc = " This struct brings together a constraint set and a (normal, not"] # [doc = " reverse) constraint graph. It implements the graph traits and is"] # [doc = " usd for doing the SCC computation."] pub (crate) struct RegionGraph < 'a , 'tcx , D : ConstraintGraphDirection > { set : & 'a OutlivesConstraintSet < 'tcx > , constraint_graph : & 'a ConstraintGraph < D > , static_region : RegionVid , }
    };
}

RegionGraph!();