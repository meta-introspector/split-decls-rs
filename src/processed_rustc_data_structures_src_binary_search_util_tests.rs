/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_binary_search_util_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: * ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_binary_search_util_tests_TYPE_0002
/* FP:tests.rs-0004 */ type Element = (usize , & 'static str) ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_binary_search_util_tests_FN_0003
/* FP:tests.rs-0006 */ fn test_map () -> Vec < Element > { let mut data = vec ! [(3 , "three-a") , (0 , "zero") , (3 , "three-b") , (22 , "twenty-two")] ; data . sort_by_key (get_key) ; data }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_binary_search_util_tests_FN_0004
/* FP:tests.rs-0008 */ fn get_key (data : & Element) -> usize { data . 0 }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_binary_search_util_tests_FN_0005
/* FP:tests.rs-0010 */ # [test] fn binary_search_slice_test () { let map = test_map () ; assert_eq ! (binary_search_slice (& map , get_key , & 0) , & [(0 , "zero")]) ; assert_eq ! (binary_search_slice (& map , get_key , & 1) , & []) ; assert_eq ! (binary_search_slice (& map , get_key , & 3) , & [(3 , "three-a") , (3 , "three-b")]) ; assert_eq ! (binary_search_slice (& map , get_key , & 22) , & [(22 , "twenty-two")]) ; assert_eq ! (binary_search_slice (& map , get_key , & 23) , & []) ; }