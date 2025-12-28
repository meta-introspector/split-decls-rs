macro_rules! deps {
    () => {
        Directed!();
        DefaultIx!();
        StableGraph!();
    };
}

macro_rules! StableDiGraph {
    () => {
        deps!();
        # [doc = " A `StableGraph` with directed edges."] # [doc = ""] # [doc = " For example, an edge from *1* to *2* is distinct from an edge from *2* to"] # [doc = " *1*."] pub type StableDiGraph < N , E , Ix = DefaultIx > = StableGraph < N , E , Directed , Ix > ;
    };
}

StableDiGraph!()