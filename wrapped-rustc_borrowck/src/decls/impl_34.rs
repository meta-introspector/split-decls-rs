macro_rules! deps {
    () => {
        ConstraintGraphDirection!();
        RegionGraph!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a , 'tcx , D : ConstraintGraphDirection > graph :: DirectedGraph for RegionGraph < 'a , 'tcx , D > { type Node = RegionVid ; fn num_nodes (& self) -> usize { self . constraint_graph . first_constraints . len () } }
    };
}

impl_34!()