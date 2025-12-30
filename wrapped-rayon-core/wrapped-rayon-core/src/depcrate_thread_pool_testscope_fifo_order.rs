// Generated macro for scope_fifo_order (function)
macro_rules! Depcrate_thread_pool_testscope_fifo_order {
() => {
// Module: crate::thread_pool::test
// Provides: {"scope_fifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn scope_fifo_order () { let vec = test_scope_order ! (scope_fifo => spawn_fifo) ; let expected : Vec < i32 > = (0 .. 10) . collect () ; assert_eq ! (vec , expected) ; }
};
}
