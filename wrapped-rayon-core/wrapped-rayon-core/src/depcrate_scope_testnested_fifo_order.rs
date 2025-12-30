// Generated macro for nested_fifo_order (function)
macro_rules! Depcrate_scope_testnested_fifo_order {
() => {
// Module: crate::scope::test
// Provides: {"nested_fifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_fifo_order () { let vec = test_nested_order ! (scope_fifo => spawn_fifo , scope_fifo => spawn_fifo) ; let expected : Vec < i32 > = (0 .. 100) . collect () ; assert_eq ! (vec , expected) ; }
};
}
