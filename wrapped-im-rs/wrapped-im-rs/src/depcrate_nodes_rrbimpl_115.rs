// Generated macro for impl_115 (impl)
macro_rules! Depcrate_nodes_rrbimpl_115 {
() => {
// Module: crate::nodes::rrb
// Provides: {"impl_115"}
// Dependencies: {}
impl < A : Clone > Entry < A > { fn len (& self) -> usize { match self { Nodes (_ , ref nodes) => nodes . len () , Values (ref values) => values . len () , Empty => 0 , } } fn is_full (& self) -> bool { match self { Nodes (_ , ref nodes) => nodes . is_full () , Values (ref values) => values . is_full () , Empty => false , } } fn unwrap_values (& self) -> & Chunk < A > { match self { Values (ref values) => values , _ => panic ! ("rrb::Entry::unwrap_values: expected values, found nodes") , } } fn unwrap_nodes (& self) -> & Chunk < Node < A > > { match self { Nodes (_ , ref nodes) => nodes , _ => panic ! ("rrb::Entry::unwrap_nodes: expected nodes, found values") , } } fn unwrap_values_mut (& mut self , pool : & RRBPool < A >) -> & mut Chunk < A > { match self { Values (ref mut values) => PoolRef :: make_mut (& pool . value_pool , values) , _ => panic ! ("rrb::Entry::unwrap_values_mut: expected values, found nodes") , } } fn unwrap_nodes_mut (& mut self , pool : & RRBPool < A >) -> & mut Chunk < Node < A > > { match self { Nodes (_ , ref mut nodes) => PoolRef :: make_mut (& pool . node_pool , nodes) , _ => panic ! ("rrb::Entry::unwrap_nodes_mut: expected nodes, found values") , } } fn values (self) -> Chunk < A > { match self { Values (values) => PoolRef :: unwrap_or_clone (values) , _ => panic ! ("rrb::Entry::values: expected values, found nodes") , } } fn nodes (self) -> Chunk < Node < A > > { match self { Nodes (_ , nodes) => PoolRef :: unwrap_or_clone (nodes) , _ => panic ! ("rrb::Entry::nodes: expected nodes, found values") , } } fn is_empty_node (& self) -> bool { matches ! (self , Empty) } }
};
}
