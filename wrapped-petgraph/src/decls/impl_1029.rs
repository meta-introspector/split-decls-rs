macro_rules! deps {
    () => {
        EdgeType!();
        Nullable!();
        IndexType!();
        MatrixGraph!();
    };
}

macro_rules! impl_1029 {
    () => {
        deps!();
        impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Visitable for MatrixGraph < N , E , S , Ty , Null , Ix > { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_bound ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_bound ()) ; } }
    };
}

impl_1029!()