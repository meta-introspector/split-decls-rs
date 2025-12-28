macro_rules! deps {
    () => {
        IntoWeightedEdge!();
        EdgeType!();
        GraphMap!();
        Create!();
        NodeTrait!();
    };
}

macro_rules! impl_889 {
    () => {
        deps!();
        # [doc = " Create a new `GraphMap` from an iterable of edges."] impl < N , E , Ty , Item , S > FromIterator < Item > for GraphMap < N , E , Ty , S > where Item : IntoWeightedEdge < E , NodeId = N > , N : NodeTrait , Ty : EdgeType , S : BuildHasher + Default , { fn from_iter < I > (iterable : I) -> Self where I : IntoIterator < Item = Item > , { let iter = iterable . into_iter () ; let (low , _) = iter . size_hint () ; let mut g = Self :: with_capacity (0 , low) ; g . extend (iter) ; g } }
    };
}

impl_889!();