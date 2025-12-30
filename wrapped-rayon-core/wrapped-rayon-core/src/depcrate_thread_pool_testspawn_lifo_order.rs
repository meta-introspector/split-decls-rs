// Generated macro for spawn_lifo_order (function)
macro_rules! Depcrate_thread_pool_testspawn_lifo_order {
() => {
// Module: crate::thread_pool::test
// Provides: {"spawn_lifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn spawn_lifo_order () { let vec = test_spawn_order ! (spawn) ; let expected : Vec < i32 > = (0 .. 10) . rev () . collect () ; assert_eq ! (vec , expected) ; }
};
}
