/* FP:reversed.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reversed_USE_0001
/* FP:reversed.rs-0002 */ use crate :: graph :: { DirectedGraph , Predecessors , Successors } ;
/* FP:reversed.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reversed_STRUCT_0002
/* FP:reversed.rs-0004 */ # [doc = " View that reverses the direction of edges in its underlying graph, so that"] # [doc = " successors become predecessors and vice-versa."] # [doc = ""] # [doc = " Because of `impl<G: Graph> Graph for &G`, the underlying graph can be"] # [doc = " wrapped by-reference instead of by-value if desired."] # [derive (Clone , Copy , Debug)] pub struct ReversedGraph < G > { pub inner : G , }
/* FP:reversed.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reversed_IMPL_0003
/* FP:reversed.rs-0006 */ impl < G > ReversedGraph < G > { pub fn new (inner : G) -> Self { Self { inner } } }
/* FP:reversed.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reversed_IMPL_0004
/* FP:reversed.rs-0008 */ impl < G : DirectedGraph > DirectedGraph for ReversedGraph < G > { type Node = G :: Node ; fn num_nodes (& self) -> usize { self . inner . num_nodes () } }
/* FP:reversed.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reversed_IMPL_0005
/* FP:reversed.rs-0010 */ impl < G : Predecessors > Successors for ReversedGraph < G > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . predecessors (node) } }
/* FP:reversed.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_reversed_IMPL_0006
/* FP:reversed.rs-0012 */ impl < G : Successors > Predecessors for ReversedGraph < G > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . successors (node) } }