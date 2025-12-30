// Generated macro for lifo_order (function)
macro_rules! Depcrate_scope_testlifo_order {
() => {
// Module: crate::scope::test
// Provides: {"lifo_order"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn lifo_order () { let vec = test_order ! (scope => spawn) ; let expected : Vec < i32 > = (0 .. 100) . rev () . collect () ; assert_eq ! (vec , expected) ; }
};
}
