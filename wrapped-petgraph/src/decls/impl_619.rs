macro_rules! deps {
    () => {
        IndexType!();
        Undirected!();
        StableGraph!();
        ToGraph6!();
    };
}

macro_rules! impl_619 {
    () => {
        deps!();
        # [cfg (feature = "stable_graph")] impl < N , E , Ix : IndexType > ToGraph6 for StableGraph < N , E , Undirected , Ix > { fn graph6_string (& self) -> String { get_graph6_representation (self) } }
    };
}

impl_619!();