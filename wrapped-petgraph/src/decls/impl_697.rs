macro_rules! deps {
    () => {
        Direction!();
        EdgesDirected!();
        EdgeType!();
        IndexType!();
        Graph!();
        Edges!();
    };
}

macro_rules! impl_697 {
    () => {
        deps!();
        impl < 'a , N , E , Ty , Ix > visit :: IntoEdgesDirected for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type EdgesDirected = Edges < 'a , E , Ty , Ix > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { self . edges_directed (a , dir) } }
    };
}

impl_697!();