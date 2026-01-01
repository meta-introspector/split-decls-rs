/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_intern_tests_USE_0001
/* FP:tests.rs-0002 */ use super :: * ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_intern_tests_STRUCT_0002
/* FP:tests.rs-0004 */ # [derive (Debug)] struct S (u32) ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_intern_tests_IMPL_0003
/* FP:tests.rs-0006 */ impl PartialEq for S { fn eq (& self , _other : & Self) -> bool { panic ! ("shouldn't be called") ; } }
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_intern_tests_IMPL_0004
/* FP:tests.rs-0008 */ impl Eq for S { }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_intern_tests_IMPL_0005
/* FP:tests.rs-0010 */ impl PartialOrd for S { fn partial_cmp (& self , other : & S) -> Option < Ordering > { assert_ne ! (self . 0 , other . 0) ; self . 0 . partial_cmp (& other . 0) } }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_intern_tests_IMPL_0006
/* FP:tests.rs-0012 */ impl Ord for S { fn cmp (& self , other : & S) -> Ordering { assert_ne ! (self . 0 , other . 0) ; self . 0 . cmp (& other . 0) } }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_intern_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn test_uniq () { let s1 = S (1) ; let s2 = S (2) ; let s3 = S (3) ; let s4 = S (1) ; let v1 = Interned :: new_unchecked (& s1) ; let v2 = Interned :: new_unchecked (& s2) ; let v3a = Interned :: new_unchecked (& s3) ; let v3b = Interned :: new_unchecked (& s3) ; let v4 = Interned :: new_unchecked (& s4) ; assert_ne ! (v1 , v2) ; assert_ne ! (v2 , v3a) ; assert_eq ! (v1 , v1) ; assert_eq ! (v3a , v3b) ; assert_ne ! (v1 , v4) ; assert_eq ! (v1 . cmp (& v2) , Ordering :: Less) ; assert_eq ! (v3a . cmp (& v2) , Ordering :: Greater) ; assert_eq ! (v1 . cmp (& v1) , Ordering :: Equal) ; assert_eq ! (v3a . cmp (& v3b) , Ordering :: Equal) ; assert_eq ! (v1 . partial_cmp (& v2) , Some (Ordering :: Less)) ; assert_eq ! (v3a . partial_cmp (& v2) , Some (Ordering :: Greater)) ; assert_eq ! (v1 . partial_cmp (& v1) , Some (Ordering :: Equal)) ; assert_eq ! (v3a . partial_cmp (& v3b) , Some (Ordering :: Equal)) ; }