// Generated macro for fifo_order (function)
macro_rules! Depcrate_scope_testfifo_order {
() => {
// Module: crate::scope::test
// Provides: {"fifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn fifo_order () { let vec = test_order ! (scope_fifo => spawn_fifo) ; let expected : Vec < i32 > = (0 .. 100) . collect () ; assert_eq ! (vec , expected) ; }
};
}
