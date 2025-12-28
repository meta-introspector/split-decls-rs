macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
        NodeTrait!();
    };
}

macro_rules! impl_935 {
    () => {
        deps!();
        impl < N , E , Ty , S > visit :: EdgeIndexable for GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { fn edge_bound (& self) -> usize { self . edge_count () } fn to_index (& self , ix : Self :: EdgeId) -> usize { self . edges . get_index_of (& ix) . expect ("edge not found") } fn from_index (& self , ix : usize) -> Self :: EdgeId { assert ! (ix < self . edges . len () , "The requested index {ix} is out-of-bounds.") ; let (& key , _) = self . edges . get_index (ix) . unwrap () ; key } }
    };
}

impl_935!()