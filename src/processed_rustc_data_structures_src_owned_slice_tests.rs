/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_USE_0001
/* FP:tests.rs-0002 */ use std :: ops :: Deref ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_USE_0002
/* FP:tests.rs-0004 */ use std :: sync :: Arc ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_USE_0003
/* FP:tests.rs-0006 */ use std :: sync :: atomic :: { self , AtomicBool } ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_USE_0004
/* FP:tests.rs-0008 */ use crate :: defer ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_USE_0005
/* FP:tests.rs-0010 */ use crate :: owned_slice :: { OwnedSlice , slice_owned , try_slice_owned } ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0006
/* FP:tests.rs-0012 */ # [test] fn smoke () { let slice = slice_owned (vec ! [1 , 2 , 3 , 4 , 5 , 6] , Vec :: as_slice) ; assert_eq ! (&* slice , [1 , 2 , 3 , 4 , 5 , 6]) ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0007
/* FP:tests.rs-0014 */ # [test] fn static_storage () { let slice = slice_owned (Box :: new (String :: from ("what")) , | _ | b"bytes boo") ; assert_eq ! (&* slice , b"bytes boo") ; }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0008
/* FP:tests.rs-0016 */ # [test] fn slice_owned_the_slice () { let slice = slice_owned (vec ! [1 , 2 , 3 , 4 , 5 , 6] , Vec :: as_slice) ; let slice = slice_owned (slice , | s | & s [1 ..] [.. 4]) ; let slice = slice_owned (slice , | s | s) ; let slice = slice_owned (slice , | s | & s [1 ..]) ; assert_eq ! (&* slice , & [1 , 2 , 3 , 4 , 5 , 6] [1 ..] [.. 4] [1 ..]) ; }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0009
/* FP:tests.rs-0018 */ # [test] fn slice_the_slice () { let slice = slice_owned (vec ! [1 , 2 , 3 , 4 , 5 , 6] , Vec :: as_slice) . slice (| s | & s [1 ..] [.. 4]) . slice (| s | s) . slice (| s | & s [1 ..]) ; assert_eq ! (&* slice , & [1 , 2 , 3 , 4 , 5 , 6] [1 ..] [.. 4] [1 ..]) ; }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0010
/* FP:tests.rs-0020 */ # [test] fn try_and_fail () { let res = try_slice_owned (vec ! [0] , | v | v . get (12 ..) . ok_or (())) ; assert ! (res . is_err ()) ; }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0011
/* FP:tests.rs-0022 */ # [test] fn boxed () { let boxed : Box < [u8] > = vec ! [1 , 1 , 2 , 3 , 5 , 8 , 13 , 21] . into_boxed_slice () ; let slice = slice_owned (boxed , Deref :: deref) ; assert_eq ! (&* slice , [1 , 1 , 2 , 3 , 5 , 8 , 13 , 21]) ; }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0012
/* FP:tests.rs-0024 */ # [test] fn drop_drops () { let flag = Arc :: new (AtomicBool :: new (false)) ; let flag_prime = Arc :: clone (& flag) ; let d = defer (move | | flag_prime . store (true , atomic :: Ordering :: Relaxed)) ; let slice = slice_owned (d , | _ | & []) ; assert_eq ! (flag . load (atomic :: Ordering :: Relaxed) , false) ; drop (slice) ; assert_eq ! (flag . load (atomic :: Ordering :: Relaxed) , true) ; }
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_owned_slice_tests_FN_0013
/* FP:tests.rs-0026 */ # [test] fn send_sync () { crate :: sync :: assert_dyn_send :: < OwnedSlice > () ; crate :: sync :: assert_dyn_sync :: < OwnedSlice > () ; }