macro_rules! deps {
    () => {
        VisitMap!();
        FilterNode!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < N > FilterNode < N > for & FixedBitSet where FixedBitSet : VisitMap < N > , { fn include_node (& self , n : N) -> bool { self . is_visited (& n) } }
    };
}

impl_115!()