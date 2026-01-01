/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: cmp :: max ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_USE_0002
/* FP:tests.rs-0004 */ use super :: * ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: fx :: FxHashMap ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_STRUCT_0004
/* FP:tests.rs-0008 */ pub (super) struct TestGraph { num_nodes : usize , start_node : usize , successors : FxHashMap < usize , Vec < usize > > , predecessors : FxHashMap < usize , Vec < usize > > , }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_IMPL_0005
/* FP:tests.rs-0010 */ impl TestGraph { pub (super) fn new (start_node : usize , edges : & [(usize , usize)]) -> Self { let mut graph = TestGraph { num_nodes : start_node + 1 , start_node , successors : FxHashMap :: default () , predecessors : FxHashMap :: default () , } ; for & (source , target) in edges { graph . num_nodes = max (graph . num_nodes , source + 1) ; graph . num_nodes = max (graph . num_nodes , target + 1) ; graph . successors . entry (source) . or_default () . push (target) ; graph . predecessors . entry (target) . or_default () . push (source) ; } for node in 0 .. graph . num_nodes { graph . successors . entry (node) . or_default () ; graph . predecessors . entry (node) . or_default () ; } graph } }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_IMPL_0006
/* FP:tests.rs-0012 */ impl DirectedGraph for TestGraph { type Node = usize ; fn num_nodes (& self) -> usize { self . num_nodes } }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_IMPL_0007
/* FP:tests.rs-0014 */ impl StartNode for TestGraph { fn start_node (& self) -> usize { self . start_node } }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_IMPL_0008
/* FP:tests.rs-0016 */ impl Predecessors for TestGraph { fn predecessors (& self , node : usize) -> impl Iterator < Item = Self :: Node > { self . predecessors [& node] . iter () . cloned () } }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_tests_IMPL_0009
/* FP:tests.rs-0018 */ impl Successors for TestGraph { fn successors (& self , node : usize) -> impl Iterator < Item = Self :: Node > { self . successors [& node] . iter () . cloned () } }