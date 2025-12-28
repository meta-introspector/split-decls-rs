macro_rules! deps {
    () => {
        Direction!();
        NeighborsDirected!();
        Reversed!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < G > IntoNeighborsDirected for Reversed < G > where G : IntoNeighborsDirected , { type NeighborsDirected = G :: NeighborsDirected ; fn neighbors_directed (self , n : G :: NodeId , d : Direction) -> G :: NeighborsDirected { self . 0 . neighbors_directed (n , d . opposite ()) } }
    };
}

impl_173!();