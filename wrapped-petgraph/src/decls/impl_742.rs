macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        Neighbors!();
        NodeIndex!();
        Graph!();
    };
}

macro_rules! impl_742 {
    () => {
        deps!();
        impl < 'a , N , E : 'a , Ty , Ix > visit :: IntoNeighbors for & 'a Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Neighbors = Neighbors < 'a , E , Ix > ; fn neighbors (self , n : NodeIndex < Ix >) -> Neighbors < 'a , E , Ix > { Graph :: neighbors (self , n) } }
    };
}

impl_742!();