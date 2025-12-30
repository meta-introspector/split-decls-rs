// Generated macro for noop_result_cache (function)
macro_rules! Depcrate_test_runner_result_cachenoop_result_cache {
() => {
// Module: crate::test_runner::result_cache
// Provides: {"noop_result_cache"}
// Dependencies: {}
# [doc = " A result cache that does nothing."] # [doc = ""] # [doc = " This is the default value of `ProptestConfig.result_cache`."] pub fn noop_result_cache () -> Box < dyn ResultCache > { Box :: new (NoOpResultCache) }
};
}
