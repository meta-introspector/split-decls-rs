macro_rules! deps {
    () => {
        IndexType!();
        List!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < E , Ix > visit :: Visitable for List < E , Ix > where Ix : IndexType , { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_count ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_count ()) ; } }
    };
}

impl_301!();