/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_vec_tests_USE_0001
/* FP:tests.rs-0002 */ use crate as rustc_index ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_vec_tests_MACRO_0002
/* FP:tests.rs-0004 */ crate :: newtype_index ! { # [orderable] # [max = 0xFFFF_FFFA] struct MyIdx { } }
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_vec_tests_FN_0003
/* FP:tests.rs-0006 */ # [test] fn index_size_is_optimized () { assert_eq ! (size_of ::< MyIdx > () , 4) ; assert_eq ! (size_of ::< Option < MyIdx >> () , 4) ; assert_eq ! (size_of ::< Option < Option < MyIdx >>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < MyIdx >>>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < Option < MyIdx >>>>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < Option < Option < MyIdx >>>>>> () , 4) ; assert_eq ! (size_of ::< Option < Option < Option < Option < Option < Option < MyIdx >>>>>>> () , 8) ; }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_vec_tests_FN_0004
/* FP:tests.rs-0008 */ # [test] fn range_iterator_iterates_forwards () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . collect ::< Vec < _ >> () , [MyIdx :: from_u32 (1) , MyIdx :: from_u32 (2) , MyIdx :: from_u32 (3)]) ; }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_vec_tests_FN_0005
/* FP:tests.rs-0010 */ # [test] fn range_iterator_iterates_backwards () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . rev () . collect ::< Vec < _ >> () , [MyIdx :: from_u32 (3) , MyIdx :: from_u32 (2) , MyIdx :: from_u32 (1)]) ; }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_vec_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn range_count_is_correct () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . count () , 3) ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_vec_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn range_size_hint_is_correct () { let range = MyIdx :: from_u32 (1) .. MyIdx :: from_u32 (4) ; assert_eq ! (range . size_hint () , (3 , Some (3))) ; }