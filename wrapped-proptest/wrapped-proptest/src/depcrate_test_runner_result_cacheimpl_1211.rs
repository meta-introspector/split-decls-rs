// Generated macro for impl_1211 (impl)
macro_rules! Depcrate_test_runner_result_cacheimpl_1211 {
() => {
// Module: crate::test_runner::result_cache
// Provides: {"impl_1211"}
// Dependencies: {}
impl ResultCache for NoOpResultCache { fn key (& self , _ : & ResultCacheKey) -> u64 { 0 } fn put (& mut self , _ : u64 , _ : & TestCaseResult) { } fn get (& self , _ : u64) -> Option < & TestCaseResult > { None } }
};
}
