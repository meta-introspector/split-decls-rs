// Generated macro for spawn_then_join_in_worker (function)
macro_rules! Depcrate_spawn_testspawn_then_join_in_worker {
() => {
// Module: crate::spawn::test
// Provides: {"spawn_then_join_in_worker"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn spawn_then_join_in_worker () { let (tx , rx) = channel () ; scope (move | _ | { spawn (move | | tx . send (22) . unwrap ()) ; }) ; assert_eq ! (22 , rx . recv () . unwrap ()) ; }
};
}
