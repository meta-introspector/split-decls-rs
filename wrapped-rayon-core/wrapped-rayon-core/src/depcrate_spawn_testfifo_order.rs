// Generated macro for fifo_order (function)
macro_rules! Depcrate_spawn_testfifo_order {
() => {
// Module: crate::spawn::test
// Provides: {"fifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn fifo_order () { let vec = test_order ! (spawn_fifo , spawn_fifo) ; let expected : Vec < i32 > = (0 .. 100) . collect () ; assert_eq ! (vec , expected) ; }
};
}
