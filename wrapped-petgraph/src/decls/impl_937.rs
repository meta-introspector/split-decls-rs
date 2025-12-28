macro_rules! deps {
    () => {
        EdgesDirected!();
        EdgeType!();
        NodeTrait!();
        Direction!();
        GraphMap!();
    };
}

macro_rules! impl_937 {
    () => {
        deps!();
        impl < 'a , N : 'a , E : 'a , Ty , S > visit :: IntoEdgesDirected for & 'a GraphMap < N , E , Ty , S > where N : NodeTrait , Ty : EdgeType , S : BuildHasher , { type EdgesDirected = EdgesDirected < 'a , N , E , Ty , S > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { self . edges_directed (a , dir) } }
    };
}

impl_937!()