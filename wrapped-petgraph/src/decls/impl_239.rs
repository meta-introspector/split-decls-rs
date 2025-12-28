macro_rules! deps {
    () => {
        Create!();
        StableGraph!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < N , E , Ty , Ix > Create for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn with_capacity (nodes : usize , edges : usize) -> Self { Self :: with_capacity (nodes , edges) } }
    };
}

impl_239!();