macro_rules! deps {
    () => {
        IndexType!();
        EdgeType!();
        StableGraph!();
    };
}

macro_rules! impl_849 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > visit :: Visitable for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_bound ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_bound ()) ; } }
    };
}

impl_849!()