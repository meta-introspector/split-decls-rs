/* FP:scope_join.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_scope_join_FN_0001
/* FP:scope_join.rs-0002 */ # [allow (unused_crate_dependencies)] # [doc = " Test that one can emulate join with `scope`:"] fn pseudo_join < F , G > (f : F , g : G) where F : FnOnce () + Send , G : FnOnce () + Send , { crate :: rustc_thread_pool :: scope (| s | { s . spawn (| _ | g ()) ; f () ; }) ; }
/* FP:scope_join.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_scope_join_FN_0002
/* FP:scope_join.rs-0004 */ fn quick_sort < T : PartialOrd + Send > (v : & mut [T]) { if v . len () <= 1 { return ; } let mid = partition (v) ; let (lo , hi) = v . split_at_mut (mid) ; pseudo_join (| | quick_sort (lo) , | | quick_sort (hi)) ; }
/* FP:scope_join.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_scope_join_FN_0003
/* FP:scope_join.rs-0006 */ fn partition < T : PartialOrd + Send > (v : & mut [T]) -> usize { let pivot = v . len () - 1 ; let mut i = 0 ; for j in 0 .. pivot { if v [j] <= v [pivot] { v . swap (i , j) ; i += 1 ; } } v . swap (i , pivot) ; i }
/* FP:scope_join.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_scope_join_FN_0004
/* FP:scope_join.rs-0008 */ fn is_sorted < T : Send + Ord > (v : & [T]) -> bool { (1 .. v . len ()) . all (| i | v [i - 1] <= v [i]) }
/* FP:scope_join.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_thread_pool_tests_scope_join_FN_0005
/* FP:scope_join.rs-0010 */ # [test] fn scope_join () { let mut v : Vec < i32 > = (0 .. 256) . rev () . collect () ; quick_sort (& mut v) ; assert ! (is_sorted (& v)) ; }