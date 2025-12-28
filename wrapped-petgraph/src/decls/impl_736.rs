macro_rules! deps {
    () => {
        Graph!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_736 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: Visitable for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_count ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_count ()) ; } }
    };
}

impl_736!();