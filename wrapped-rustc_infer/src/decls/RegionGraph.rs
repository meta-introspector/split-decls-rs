macro_rules! deps {
    () => {
        Constraint!();
    };
}

macro_rules! RegionGraph {
    () => {
        deps!();
        type RegionGraph < 'tcx > = LinkedGraph < () , Constraint < 'tcx > > ;
    };
}

RegionGraph!()