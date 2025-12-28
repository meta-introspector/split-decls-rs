macro_rules! deps {
    () => {
        GraphMap!();
        EdgeType!();
        NodeTrait!();
        IntoWeightedEdge!();
    };
}

macro_rules! impl_890 {
    () => {
        deps!();
        # [doc = " Extend the graph from an iterable of edges."] # [doc = ""] # [doc = " Nodes are inserted automatically to match the edges."] impl < N , E , Ty , Item , S > Extend < Item > for GraphMap < N , E , Ty , S > where Item : IntoWeightedEdge < E , NodeId = N > , N : NodeTrait , Ty : EdgeType , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = Item > , { let iter = iterable . into_iter () ; let (low , _) = iter . size_hint () ; self . edges . reserve (low) ; for elt in iter { let (source , target , weight) = elt . into_weighted_edge () ; self . add_edge (source , target , weight) ; } } }
    };
}

impl_890!()