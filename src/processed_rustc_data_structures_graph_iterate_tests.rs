/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_iterate_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: super :: tests :: TestGraph ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_iterate_tests_USE_0002
/* FP:tests.rs-0004 */ use super :: * ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_iterate_tests_FN_0003
/* FP:tests.rs-0006 */ # [test] fn diamond_post_order () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)]) ; let result = post_order_from (& graph , 0) ; assert_eq ! (result , vec ! [3 , 1 , 2 , 0]) ; }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_iterate_tests_FN_0004
/* FP:tests.rs-0008 */ # [test] fn is_cyclic () { use super :: super :: is_cyclic ; let diamond_acyclic = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)]) ; let diamond_cyclic = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (2 , 3) , (3 , 0)]) ; assert ! (! is_cyclic (& diamond_acyclic)) ; assert ! (is_cyclic (& diamond_cyclic)) ; }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_iterate_tests_FN_0005
/* FP:tests.rs-0010 */ # [test] fn dfs () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3) , (3 , 0)]) ; let result : Vec < usize > = DepthFirstSearch :: new (& graph) . with_start_node (0) . collect () ; assert_eq ! (result , vec ! [0 , 2 , 3 , 1]) ; }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_graph_iterate_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn dfs_debug () { let graph = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3) , (3 , 0)]) ; let mut dfs = DepthFirstSearch :: new (& graph) . with_start_node (0) ; dfs . complete_search () ; assert_eq ! (format ! ("{{0, 1, 2, 3}}") , format ! ("{:?}" , dfs)) ; }