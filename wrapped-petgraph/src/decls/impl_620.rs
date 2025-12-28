macro_rules! deps {
    () => {
        ToGraph6!();
        GraphMap!();
        NodeTrait!();
        Undirected!();
    };
}

macro_rules! impl_620 {
    () => {
        deps!();
        # [cfg (feature = "graphmap")] impl < N : NodeTrait , E , S : BuildHasher > ToGraph6 for GraphMap < N , E , Undirected , S > { fn graph6_string (& self) -> String { get_graph6_representation (self) } }
    };
}

impl_620!()