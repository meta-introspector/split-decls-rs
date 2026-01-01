/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_abi_src_extern_abi_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: assert_matches :: assert_matches ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_abi_src_extern_abi_tests_USE_0002
/* FP:tests.rs-0004 */ use std :: str :: FromStr ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_abi_src_extern_abi_tests_USE_0003
/* FP:tests.rs-0006 */ use super :: * ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_abi_src_extern_abi_tests_FN_0004
/* FP:tests.rs-0008 */ # [allow (non_snake_case)] # [test] fn lookup_Rust () { let abi = ExternAbi :: from_str ("Rust") ; assert ! (abi . is_ok () && abi . unwrap () . as_str () == "Rust") ; }
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_abi_src_extern_abi_tests_FN_0005
/* FP:tests.rs-0010 */ # [test] fn lookup_cdecl () { let abi = ExternAbi :: from_str ("cdecl") ; assert ! (abi . is_ok () && abi . unwrap () . as_str () == "cdecl") ; }
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_abi_src_extern_abi_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn lookup_baz () { let abi = ExternAbi :: from_str ("baz") ; assert_matches ! (abi , Err (AbiFromStrErr :: Unknown)) ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_abi_src_extern_abi_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn guarantee_lexicographic_ordering () { let abis = ExternAbi :: ALL_VARIANTS ; let mut sorted_abis = abis . to_vec () ; sorted_abis . sort_unstable () ; assert_eq ! (abis , sorted_abis) ; }