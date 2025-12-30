// Generated macro for impl_174 (impl)
macro_rules! Depcrate_graph_linked_graphimpl_174 {
() => {
// Module: crate::graph::linked_graph
// Provides: {"impl_174"}
// Dependencies: {}
impl < 'g , N : Debug , E : Debug > DepthFirstTraversal < 'g , N , E > { pub fn with_start_node (graph : & 'g LinkedGraph < N , E > , start_node : NodeIndex , direction : Direction ,) -> Self { let mut visited = DenseBitSet :: new_empty (graph . len_nodes ()) ; visited . insert (start_node . node_id ()) ; DepthFirstTraversal { graph , stack : vec ! [start_node] , visited , direction } } fn visit (& mut self , node : NodeIndex) { if self . visited . insert (node . node_id ()) { self . stack . push (node) ; } } }
};
}
