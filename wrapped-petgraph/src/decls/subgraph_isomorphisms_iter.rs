macro_rules! deps {
    () => {
        EdgeType!();
    };
}

macro_rules! subgraph_isomorphisms_iter {
    () => {
        deps!();
        # [doc = " Using the VF2 algorithm, examine both syntactic and semantic graph"] # [doc = " isomorphism (graph structure and matching node and edge weights) and,"] # [doc = " if `g0` is isomorphic to a subgraph of `g1`, return the mappings between"] # [doc = " them."] # [doc = ""] # [doc = " The graphs should not be [multigraphs]."] # [doc = ""] # [doc = " [multigraphs]: https://en.wikipedia.org/wiki/Multigraph"] pub fn subgraph_isomorphisms_iter < 'a , G0 , G1 , NM , EM > (g0 : & 'a G0 , g1 : & 'a G1 , node_match : & 'a mut NM , edge_match : & 'a mut EM ,) -> Option < impl Iterator < Item = Vec < usize > > + 'a > where G0 : 'a + NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp + IntoEdgesDirected , G1 : 'a + NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp < EdgeType = G0 :: EdgeType > + IntoEdgesDirected , NM : 'a + FnMut (& G0 :: NodeWeight , & G1 :: NodeWeight) -> bool , EM : 'a + FnMut (& G0 :: EdgeWeight , & G1 :: EdgeWeight) -> bool , { if g0 . node_count () > g1 . node_count () || g0 . edge_count () > g1 . edge_count () { return None ; } Some (self :: matching :: GraphMatcher :: new (g0 , g1 , node_match , edge_match , true ,)) }
    };
}

subgraph_isomorphisms_iter!();