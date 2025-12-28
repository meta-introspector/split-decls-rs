macro_rules! deps {
    () => {
        Direction!();
        Edges!();
        StableGraph!();
        IndexType!();
        EdgesDirected!();
        EdgeType!();
    };
}

macro_rules! impl_861 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > visit :: IntoEdgesDirected for & 'a StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgesDirected = Edges < 'a , E , Ty , Ix > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { self . edges_directed (a , dir) } }
    };
}

impl_861!();