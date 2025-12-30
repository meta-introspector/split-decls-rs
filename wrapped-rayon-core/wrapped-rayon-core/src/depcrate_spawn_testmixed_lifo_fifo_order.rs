// Generated macro for mixed_lifo_fifo_order (function)
macro_rules! Depcrate_spawn_testmixed_lifo_fifo_order {
() => {
// Module: crate::spawn::test
// Provides: {"mixed_lifo_fifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_lifo_fifo_order () { let vec = test_mixed_order ! (spawn , spawn_fifo) ; let expected = vec ! [3 , - 1 , 2 , - 2 , 1 , - 3 , 0] ; assert_eq ! (vec , expected) ; }
};
}
