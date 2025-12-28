macro_rules! deps {
    () => {
        MatrixGraph!();
        Nullable!();
        ToGraph6!();
        Undirected!();
        NodeTrait!();
        IndexType!();
    };
}

macro_rules! impl_621 {
    () => {
        deps!();
        # [cfg (feature = "matrix_graph")] impl < N , E , S , Null , Ix > ToGraph6 for MatrixGraph < N , E , S , Undirected , Null , Ix > where N : NodeTrait , Null : Nullable < Wrapped = E > , Ix : IndexType , S : BuildHasher + Default , { fn graph6_string (& self) -> String { get_graph6_representation (self) } }
    };
}

impl_621!();