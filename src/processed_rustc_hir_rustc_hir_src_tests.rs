/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_tests_USE_0001
/* FP:tests.rs-0002 */ # [allow (rustc :: symbol_intern_string_literal)] use rustc_hashes :: Hash64 ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_tests_USE_0002
/* FP:tests.rs-0004 */ use crate :: rustc_complete :: def_id :: { DefPathHash , StableCrateId } ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: rustc_complete :: edition :: Edition ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_tests_USE_0004
/* FP:tests.rs-0008 */ use crate :: rustc_complete :: { Symbol , create_session_globals_then } ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_tests_USE_0005
/* FP:tests.rs-0010 */ use crate :: definitions :: { DefKey , DefPathData , DisambiguatedDefPathData } ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_src_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn def_path_hash_depends_on_crate_id () { create_session_globals_then (Edition :: Edition2024 , & [] , None , | | { let id0 = StableCrateId :: new (Symbol :: intern ("foo") , false , vec ! ["1" . to_string ()] , "") ; let id1 = StableCrateId :: new (Symbol :: intern ("foo") , false , vec ! ["2" . to_string ()] , "") ; let h0 = mk_test_hash (id0) ; let h1 = mk_test_hash (id1) ; assert_ne ! (h0 . stable_crate_id () , h1 . stable_crate_id ()) ; assert_ne ! (h0 . local_hash () , h1 . local_hash ()) ; fn mk_test_hash (stable_crate_id : StableCrateId) -> DefPathHash { let parent_hash = DefPathHash :: new (stable_crate_id , Hash64 :: new (stable_crate_id . as_u64 ())) ; let key = DefKey { parent : None , disambiguated_data : DisambiguatedDefPathData { data : DefPathData :: CrateRoot , disambiguator : 0 , } , } ; key . compute_stable_hash (parent_hash) } }) }