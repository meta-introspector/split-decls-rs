macro_rules! deps {
    () => {
        DefaultIx!();
        Undirected!();
        Graph!();
    };
}

macro_rules! UnGraph {
    () => {
        deps!();
        # [doc = " A `Graph` with undirected edges."] # [doc = ""] # [doc = " For example, an edge between *1* and *2* is equivalent to an edge between"] # [doc = " *2* and *1*."] pub type UnGraph < N , E , Ix = DefaultIx > = Graph < N , E , Undirected , Ix > ;
    };
}

UnGraph!();