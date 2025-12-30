// Generated macro for scope_lifo_order (function)
macro_rules! Depcrate_thread_pool_testscope_lifo_order {
() => {
// Module: crate::thread_pool::test
// Provides: {"scope_lifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn scope_lifo_order () { let vec = test_scope_order ! (scope => spawn) ; let expected : Vec < i32 > = (0 .. 10) . rev () . collect () ; assert_eq ! (vec , expected) ; }
};
}
