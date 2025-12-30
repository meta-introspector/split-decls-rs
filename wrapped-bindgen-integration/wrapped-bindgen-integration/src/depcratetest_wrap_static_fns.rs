// Generated macro for test_wrap_static_fns (function)
macro_rules! Depcratetest_wrap_static_fns {
() => {
// Module: crate
// Provides: {"test_wrap_static_fns"}
// Dependencies: {}
# [test] fn test_wrap_static_fns () { unsafe { let f = extern_bindings :: foo () ; assert_eq ! (11 , f) ; let b = extern_bindings :: bar () ; assert_eq ! (1 , b) ; let t = extern_bindings :: takes_ptr (& mut 1) ; assert_eq ! (2 , t) ; extern "C" fn function (x : i32) -> i32 { x + 1 } let tp = extern_bindings :: takes_fn_ptr (Some (function)) ; assert_eq ! (2 , tp) ; let tf = extern_bindings :: takes_fn (Some (function)) ; assert_eq ! (3 , tf) ; let ta = extern_bindings :: takes_alias (Some (function)) ; assert_eq ! (4 , ta) ; let tq = extern_bindings :: takes_qualified (& (& 5 as * const _) as * const _) ; assert_eq ! (5 , tq) ; # [cfg (not (all (target_arch = "aarch64" , target_os = "linux")))] { let wv1 = extern_bindings :: wrap_as_variadic_fn1_wrapped (0) ; assert_eq ! (0 , wv1) ; let wv1 = extern_bindings :: wrap_as_variadic_fn1_wrapped (2 , 5 , 3) ; assert_eq ! (8 , wv1) ; extern_bindings :: wrap_as_variadic_fn2_wrapped (1 , 2) ; } } }
};
}
