// Generated macro for basic_result_cache (function)
macro_rules! Depcrate_test_runner_result_cachebasic_result_cache {
() => {
// Module: crate::test_runner::result_cache
// Provides: {"basic_result_cache"}
// Dependencies: {}
# [doc = " A basic result cache."] # [doc = ""] # [doc = " Values are identified by their `Debug` string representation."] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn basic_result_cache () -> Box < dyn ResultCache > { Box :: new (BasicResultCache :: default ()) }
};
}
