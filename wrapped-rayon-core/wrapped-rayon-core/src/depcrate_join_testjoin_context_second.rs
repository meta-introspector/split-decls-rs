// Generated macro for join_context_second (function)
macro_rules! Depcrate_join_testjoin_context_second {
() => {
// Module: crate::join::test
// Provides: {"join_context_second"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_context_second () { use std :: sync :: Barrier ; let barrier = Barrier :: new (2) ; let pool = ThreadPoolBuilder :: new () . num_threads (2) . build () . unwrap () ; let (a_migrated , b_migrated) = pool . install (| | { join_context (| a | { barrier . wait () ; a . migrated () } , | b | { barrier . wait () ; b . migrated () } ,) }) ; assert ! (! a_migrated) ; assert ! (b_migrated) ; }
};
}
