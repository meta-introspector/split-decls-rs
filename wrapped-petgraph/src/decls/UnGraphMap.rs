macro_rules! deps {
    () => {
        GraphMap!();
        Undirected!();
    };
}

macro_rules! UnGraphMap {
    () => {
        deps!();
        # [doc = " A `GraphMap` with undirected edges."] # [doc = ""] # [doc = " For example, an edge between *1* and *2* is equivalent to an edge between"] # [doc = " *2* and *1*."] pub type UnGraphMap < N , E , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState > = GraphMap < N , E , Undirected , S > ;
    };
}

UnGraphMap!()