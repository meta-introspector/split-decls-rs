macro_rules! Subgraph {
    () => {
        type Subgraph < G > = HashSet < < G as GraphBase > :: NodeId > ;
    };
}

Subgraph!()