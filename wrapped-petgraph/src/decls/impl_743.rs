macro_rules! deps {
    () => {
        Graph!();
        EdgeType!();
        NeighborsDirected!();
        Direction!();
        Neighbors!();
        NodeIndex!();
        IndexType!();
    };
}

macro_rules! impl_743 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighborsDirected for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type NeighborsDirected = Neighbors < 'a , E , Ix > ; fn neighbors_directed (self , n : NodeIndex < Ix > , d : Direction) -> Neighbors < 'a , E , Ix > { Graph :: neighbors_directed (self , n , d) } }
    };
}

impl_743!()