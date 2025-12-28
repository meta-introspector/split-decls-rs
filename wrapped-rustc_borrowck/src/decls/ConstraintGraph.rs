macro_rules! deps {
    () => {
        ConstraintGraphDirection!();
    };
}

macro_rules! ConstraintGraph {
    () => {
        deps!();
        # [doc = " The construct graph organizes the constraints by their end-points."] # [doc = " It can be used to view a `R1: R2` constraint as either an edge `R1"] # [doc = " -> R2` or `R2 -> R1` depending on the direction type `D`."] pub (crate) struct ConstraintGraph < D : ConstraintGraphDirection > { _direction : D , first_constraints : IndexVec < RegionVid , Option < OutlivesConstraintIndex > > , next_constraints : IndexVec < OutlivesConstraintIndex , Option < OutlivesConstraintIndex > > , }
    };
}

ConstraintGraph!()