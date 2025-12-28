macro_rules! deps {
    () => {
        FilterNode!();
        VisitMap!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        # [doc = " This filter includes the nodes that are contained in the set."] impl < N > FilterNode < N > for FixedBitSet where FixedBitSet : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
    };
}

impl_113!()