macro_rules! deps {
    () => {
        RegionGraph!();
        ConstraintGraphDirection!();
        Successors!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a , 'tcx , D : ConstraintGraphDirection > graph :: Successors for RegionGraph < 'a , 'tcx , D > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . outgoing_regions (node) } }
    };
}

impl_35!()