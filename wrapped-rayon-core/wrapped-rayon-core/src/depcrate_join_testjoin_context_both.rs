// Generated macro for join_context_both (function)
macro_rules! Depcrate_join_testjoin_context_both {
() => {
// Module: crate::join::test
// Provides: {"join_context_both"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_context_both () { let (a_migrated , b_migrated) = join_context (| a | a . migrated () , | b | b . migrated ()) ; assert ! (a_migrated) ; assert ! (b_migrated) ; }
};
}
