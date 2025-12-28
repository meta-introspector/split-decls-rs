macro_rules! deps {
    () => {
        Undirected!();
        StableGraph!();
        DefaultIx!();
    };
}

macro_rules! StableUnGraph {
    () => {
        deps!();
        # [doc = " A `StableGraph` with undirected edges."] # [doc = ""] # [doc = " For example, an edge between *1* and *2* is equivalent to an edge between"] # [doc = " *2* and *1*."] pub type StableUnGraph < N , E , Ix = DefaultIx > = StableGraph < N , E , Undirected , Ix > ;
    };
}

StableUnGraph!();