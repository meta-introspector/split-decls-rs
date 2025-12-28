macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
    };
}

macro_rules! impl_926 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: Visitable for GraphMap < N , E , Ty , S > where N : Copy + Ord + Hash , Ty : EdgeType , S : BuildHasher , { type Map = HashSet < N > ; fn visit_map (& self) -> HashSet < N > { HashSet :: with_capacity (self . node_count ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; } }
    };
}

impl_926!()