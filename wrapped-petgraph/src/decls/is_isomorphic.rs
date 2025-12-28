macro_rules! deps {
    () => {
        Matching!();
        EdgeType!();
    };
}

macro_rules! is_isomorphic {
    () => {
        deps!();
        # [doc = " Return `true` if the graphs `g0` and `g1` are isomorphic."] # [doc = ""] # [doc = " Using the VF2 algorithm, only matching graph syntactically (graph"] # [doc = " structure)."] # [doc = ""] # [doc = " The graphs should not be [multigraphs]."] # [doc = ""] # [doc = " **Reference**"] # [doc = ""] # [doc = " * Luigi P. Cordella, Pasquale Foggia, Carlo Sansone, Mario Vento;"] # [doc = "   *A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs*"] # [doc = ""] # [doc = " [multigraphs]: https://en.wikipedia.org/wiki/Multigraph"] pub fn is_isomorphic < G0 , G1 > (g0 : G0 , g1 : G1) -> bool where G0 : NodeCompactIndexable + EdgeCount + GetAdjacencyMatrix + GraphProp + IntoNeighborsDirected , G1 : NodeCompactIndexable + EdgeCount + GetAdjacencyMatrix + GraphProp < EdgeType = G0 :: EdgeType > + IntoNeighborsDirected , { if g0 . node_count () != g1 . node_count () || g0 . edge_count () != g1 . edge_count () { return false ; } self :: matching :: GraphMatcher :: new (& g0 , & g1 , & mut NoSemanticMatch , & mut NoSemanticMatch , false) . next () . is_some () }
    };
}

is_isomorphic!();