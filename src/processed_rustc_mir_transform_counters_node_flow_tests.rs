/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_USE_0001
/* FP:tests.rs-0002 */ use itertools :: Itertools ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: rustc_data_structures :: graph ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: rustc_data_structures :: graph :: vec_graph :: VecGraph ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_USE_0004
/* FP:tests.rs-0008 */ use crate :: rustc_index :: Idx ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_USE_0005
/* FP:tests.rs-0010 */ use crate :: rustc_complete :: mir :: coverage :: Op ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_USE_0006
/* FP:tests.rs-0012 */ use crate :: coverage :: counters :: node_flow :: { CounterTerm , NodeCounters , NodeFlowData , make_node_counters , node_flow_data_for_balanced_graph , } ;
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_FN_0007
/* FP:tests.rs-0014 */ fn node_flow_data < G : graph :: Successors > (graph : G) -> NodeFlowData < G :: Node > { node_flow_data_for_balanced_graph (graph) }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_FN_0008
/* FP:tests.rs-0016 */ fn make_graph < Node : Idx + Ord > (num_nodes : usize , edge_pairs : Vec < (Node , Node) >) -> VecGraph < Node > { VecGraph :: new (num_nodes , edge_pairs) }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_FN_0009
/* FP:tests.rs-0018 */ # [doc = " Example used in \"Optimal Measurement Points for Program Frequency Counts\""] # [doc = " (Knuth & Stevenson, 1973), but with 0-based node IDs."] # [test] fn example_driver () { let graph = make_graph :: < u32 > (5 , vec ! [(0 , 1) , (0 , 3) , (1 , 0) , (1 , 2) , (2 , 1) , (2 , 4) , (3 , 3) , (3 , 4) , (4 , 0)] ,) ; let node_flow_data = node_flow_data (& graph) ; let counters = make_node_counters (& node_flow_data , & [3 , 1 , 2 , 0 , 4]) ; assert_eq ! (format_counter_expressions (& counters) , & ["[0]: +c0" , "[1]: +c0 +c2 -c4" , "[2]: +c2" , "[3]: +c3" , "[4]: +c4" ,]) ; }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_coverage_counters_node_flow_tests_FN_0010
/* FP:tests.rs-0020 */ fn format_counter_expressions < Node : Idx > (counters : & NodeCounters < Node >) -> Vec < String > { let format_item = | & CounterTerm { node , op } | { let op = match op { Op :: Subtract => '-' , Op :: Add => '+' , } ; format ! ("{op}c{node:?}") } ; counters . counter_terms . indices () . map (| node | { let mut terms = counters . counter_terms [node] . iter () . collect :: < Vec < _ > > () ; terms . sort_by_key (| item | item . node . index ()) ; format ! ("[{node:?}]: {}" , terms . into_iter () . map (format_item) . join (" ")) }) . collect () }