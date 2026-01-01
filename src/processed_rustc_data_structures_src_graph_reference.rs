/* FP:reference.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reference_USE_0001
/* FP:reference.rs-0002 */ use super :: * ;
/* FP:reference.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reference_IMPL_0002
/* FP:reference.rs-0004 */ impl < 'graph , G : DirectedGraph > DirectedGraph for & 'graph G { type Node = G :: Node ; fn num_nodes (& self) -> usize { (* * self) . num_nodes () } }
/* FP:reference.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reference_IMPL_0003
/* FP:reference.rs-0006 */ impl < 'graph , G : StartNode > StartNode for & 'graph G { fn start_node (& self) -> Self :: Node { (* * self) . start_node () } }
/* FP:reference.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reference_IMPL_0004
/* FP:reference.rs-0008 */ impl < 'graph , G : Successors > Successors for & 'graph G { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . successors (node) } }
/* FP:reference.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reference_IMPL_0005
/* FP:reference.rs-0010 */ impl < 'graph , G : Predecessors > Predecessors for & 'graph G { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . predecessors (node) } }