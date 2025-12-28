macro_rules! deps {
    () => {
        GraphMap!();
        Create!();
        NodeTrait!();
        EdgeType!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        # [cfg (feature = "graphmap")] impl < N , E , Ty , S > Create for GraphMap < N , E , Ty , S > where Ty : EdgeType , N : NodeTrait , S : BuildHasher + Default , { fn with_capacity (nodes : usize , edges : usize) -> Self { Self :: with_capacity (nodes , edges) } }
    };
}

impl_240!()