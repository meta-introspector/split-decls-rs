// Generated macro for nested_lifo_fifo_order (function)
macro_rules! Depcrate_scope_testnested_lifo_fifo_order {
() => {
// Module: crate::scope::test
// Provides: {"nested_lifo_fifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_lifo_fifo_order () { let vec = test_nested_order ! (scope => spawn , scope_fifo => spawn_fifo) ; let expected : Vec < i32 > = (0 .. 10) . rev () . flat_map (| i | (0 .. 10) . map (move | j | i * 10 + j)) . collect () ; assert_eq ! (vec , expected) ; }
};
}
