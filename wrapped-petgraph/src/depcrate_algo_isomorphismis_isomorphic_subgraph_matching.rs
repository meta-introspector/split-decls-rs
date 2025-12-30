// Generated macro for is_isomorphic_subgraph_matching (function)
macro_rules! Depcrate_algo_isomorphismis_isomorphic_subgraph_matching {
() => {
// Module: crate::algo::isomorphism
// Provides: {"is_isomorphic_subgraph_matching"}
// Dependencies: {}
# [doc = " Return `true` if `g0` is isomorphic to a subgraph of `g1`."] # [doc = ""] # [doc = " Using the VF2 algorithm, examining both syntactic and semantic"] # [doc = " graph isomorphism (graph structure and matching node and edge weights)."] # [doc = ""] # [doc = " The graphs should not be [multigraphs]."] # [doc = ""] # [doc = " [multigraphs]: https://en.wikipedia.org/wiki/Multigraph"] pub fn is_isomorphic_subgraph_matching < G0 , G1 , NM , EM > (g0 : G0 , g1 : G1 , mut node_match : NM , mut edge_match : EM ,) -> bool where G0 : NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp + IntoEdgesDirected , G1 : NodeCompactIndexable + EdgeCount + DataMap + GetAdjacencyMatrix + GraphProp < EdgeType = G0 :: EdgeType > + IntoEdgesDirected , NM : FnMut (& G0 :: NodeWeight , & G1 :: NodeWeight) -> bool , EM : FnMut (& G0 :: EdgeWeight , & G1 :: EdgeWeight) -> bool , { if g0 . node_count () > g1 . node_count () || g0 . edge_count () > g1 . edge_count () { return false ; } self :: matching :: GraphMatcher :: new (& g0 , & g1 , & mut node_match , & mut edge_match , true) . next () . is_some () }
};
}
