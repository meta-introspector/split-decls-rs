// Generated macro for spawn_then_join_outside_worker (function)
macro_rules! Depcrate_spawn_testspawn_then_join_outside_worker {
() => {
// Module: crate::spawn::test
// Provides: {"spawn_then_join_outside_worker"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn spawn_then_join_outside_worker () { let (tx , rx) = channel () ; spawn (move | | tx . send (22) . unwrap ()) ; assert_eq ! (22 , rx . recv () . unwrap ()) ; }
};
}
