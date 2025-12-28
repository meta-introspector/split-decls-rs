macro_rules! deps {
    () => {
        Directed!();
        Graph!();
        DefaultIx!();
    };
}

macro_rules! DiGraph {
    () => {
        deps!();
        # [doc = " A `Graph` with directed edges."] # [doc = ""] # [doc = " For example, an edge from *1* to *2* is distinct from an edge from *2* to"] # [doc = " *1*."] pub type DiGraph < N , E , Ix = DefaultIx > = Graph < N , E , Directed , Ix > ;
    };
}

DiGraph!();