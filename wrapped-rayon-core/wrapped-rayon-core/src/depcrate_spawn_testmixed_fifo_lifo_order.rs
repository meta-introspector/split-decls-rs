// Generated macro for mixed_fifo_lifo_order (function)
macro_rules! Depcrate_spawn_testmixed_fifo_lifo_order {
() => {
// Module: crate::spawn::test
// Provides: {"mixed_fifo_lifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_fifo_lifo_order () { let vec = test_mixed_order ! (spawn_fifo , spawn) ; let expected = vec ! [0 , - 3 , 1 , - 2 , 2 , - 1 , 3] ; assert_eq ! (vec , expected) ; }
};
}
