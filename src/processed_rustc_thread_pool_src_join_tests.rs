/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_USE_0001
/* FP:tests.rs-0002 */ use rand :: distr :: StandardUniform ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_USE_0002
/* FP:tests.rs-0004 */ use rand :: { Rng , SeedableRng } ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_USE_0003
/* FP:tests.rs-0006 */ use rand_xorshift :: XorShiftRng ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_USE_0004
/* FP:tests.rs-0008 */ use super :: * ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_USE_0005
/* FP:tests.rs-0010 */ use crate :: ThreadPoolBuilder ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0006
/* FP:tests.rs-0012 */ fn quick_sort < T : PartialOrd + Send > (v : & mut [T]) { if v . len () <= 1 { return ; } let mid = partition (v) ; let (lo , hi) = v . split_at_mut (mid) ; join (| | quick_sort (lo) , | | quick_sort (hi)) ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0007
/* FP:tests.rs-0014 */ fn partition < T : PartialOrd + Send > (v : & mut [T]) -> usize { let pivot = v . len () - 1 ; let mut i = 0 ; for j in 0 .. pivot { if v [j] <= v [pivot] { v . swap (i , j) ; i += 1 ; } } v . swap (i , pivot) ; i }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0008
/* FP:tests.rs-0016 */ fn seeded_rng () -> XorShiftRng { let mut seed = < XorShiftRng as SeedableRng > :: Seed :: default () ; (0 ..) . zip (seed . as_mut ()) . for_each (| (i , x) | * x = i) ; XorShiftRng :: from_seed (seed) }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0009
/* FP:tests.rs-0018 */ # [test] fn sort () { let rng = seeded_rng () ; let mut data : Vec < u32 > = rng . sample_iter (& StandardUniform) . take (6 * 1024) . collect () ; let mut sorted_data = data . clone () ; sorted_data . sort () ; quick_sort (& mut data) ; assert_eq ! (data , sorted_data) ; }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0010
/* FP:tests.rs-0020 */ # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn sort_in_pool () { let rng = seeded_rng () ; let mut data : Vec < u32 > = rng . sample_iter (& StandardUniform) . take (12 * 1024) . collect () ; let pool = ThreadPoolBuilder :: new () . build () . unwrap () ; let mut sorted_data = data . clone () ; sorted_data . sort () ; pool . install (| | quick_sort (& mut data)) ; assert_eq ! (data , sorted_data) ; }
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0011
/* FP:tests.rs-0022 */ # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_a () { join (| | panic ! ("Hello, world!") , | | ()) ; }
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0012
/* FP:tests.rs-0024 */ # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_b () { join (| | () , | | panic ! ("Hello, world!")) ; }
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0013
/* FP:tests.rs-0026 */ # [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_both () { join (| | panic ! ("Hello, world!") , | | panic ! ("Goodbye, world!")) ; }
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0014
/* FP:tests.rs-0028 */ # [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_b_still_executes () { let mut x = false ; match unwind :: halt_unwinding (| | join (| | panic ! ("Hello, world!") , | | x = true)) { Ok (_) => panic ! ("failed to propagate panic from closure A,") , Err (_) => assert ! (x , "closure b failed to execute") , } }
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0015
/* FP:tests.rs-0030 */ # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_context_both () { let (a_migrated , b_migrated) = join_context (| a | a . migrated () , | b | b . migrated ()) ; assert ! (a_migrated) ; assert ! (b_migrated) ; }
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0016
/* FP:tests.rs-0032 */ # [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_context_neither () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let (a_migrated , b_migrated) = pool . install (| | join_context (| a | a . migrated () , | b | b . migrated ())) ; assert ! (! a_migrated) ; assert ! (! b_migrated) ; }
/* FP:tests.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0017
/* FP:tests.rs-0034 */ # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_context_second () { use std :: sync :: Barrier ; let barrier = Barrier :: new (2) ; let pool = ThreadPoolBuilder :: new () . num_threads (2) . build () . unwrap () ; let (a_migrated , b_migrated) = pool . install (| | { join_context (| a | { barrier . wait () ; a . migrated () } , | b | { barrier . wait () ; b . migrated () } ,) }) ; assert ! (! a_migrated) ; assert ! (b_migrated) ; }
/* FP:tests.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_src_join_tests_FN_0018
/* FP:tests.rs-0036 */ # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_counter_overflow () { const MAX : u32 = 500_000 ; let mut i = 0 ; let mut j = 0 ; let pool = ThreadPoolBuilder :: new () . num_threads (2) . build () . unwrap () ; for _ in 0 .. MAX { pool . join (| | i += 1 , | | j += 1) ; } assert_eq ! (i , MAX) ; assert_eq ! (j , MAX) ; }