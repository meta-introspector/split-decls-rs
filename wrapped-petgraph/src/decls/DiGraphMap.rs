macro_rules! deps {
    () => {
        Directed!();
        GraphMap!();
    };
}

macro_rules! DiGraphMap {
    () => {
        deps!();
        # [doc = " A `GraphMap` with directed edges."] # [doc = ""] # [doc = " For example, an edge from *1* to *2* is distinct from an edge from *2* to"] # [doc = " *1*."] pub type DiGraphMap < N , E , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState > = GraphMap < N , E , Directed , S > ;
    };
}

DiGraphMap!();