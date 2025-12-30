// Generated macro for impl_263 (impl)
macro_rules! Depcrate_graph_vec_graphimpl_263 {
() => {
// Module: crate::graph::vec_graph
// Provides: {"impl_263"}
// Dependencies: {}
impl < N : Idx + Ord , const BR : bool > VecGraph < N , BR > { pub fn new (num_nodes : usize , mut edge_pairs : Vec < (N , N) >) -> Self { let num_edges = edge_pairs . len () ; let nodes_cap = match BR { false => num_nodes + 1 , true => (num_nodes * 2) + 1 , } ; let edges_cap = match BR { false => num_edges , true => num_edges * 2 , } ; let mut node_starts = IndexVec :: with_capacity (nodes_cap) ; let mut edge_targets = Vec :: with_capacity (edges_cap) ; edge_pairs . sort () ; create_index (num_nodes , & mut edge_pairs . iter () . map (| & (src , _) | src) , & mut edge_pairs . iter () . map (| & (_ , tgt) | tgt) , & mut edge_targets , & mut node_starts ,) ; if BR { node_starts . pop () ; edge_pairs . sort_by_key (| & (src , tgt) | (tgt , src)) ; create_index (num_nodes * 2 , & mut edge_pairs . iter () . map (| & (_ , tgt) | N :: new (tgt . index () + num_nodes)) , & mut edge_pairs . iter () . map (| & (src , _) | src) , & mut edge_targets , & mut node_starts ,) ; } Self { node_starts , edge_targets } } # [doc = " Gets the successors for `source` as a slice."] pub fn successors (& self , source : N) -> & [N] { assert ! (source . index () < self . num_nodes ()) ; let start_index = self . node_starts [source] ; let end_index = self . node_starts [source . plus (1)] ; & self . edge_targets [start_index .. end_index] } }
};
}
