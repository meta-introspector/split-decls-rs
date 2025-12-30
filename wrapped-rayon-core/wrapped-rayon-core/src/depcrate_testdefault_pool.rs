// Generated macro for default_pool (function)
macro_rules! Depcrate_testdefault_pool {
() => {
// Module: crate::test
// Provides: {"default_pool"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn default_pool () { ThreadPoolBuilder :: default () . build () . unwrap () ; }
};
}
