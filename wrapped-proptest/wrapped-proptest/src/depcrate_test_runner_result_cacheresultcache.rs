// Generated macro for ResultCache (trait)
macro_rules! Depcrate_test_runner_result_cacheResultCache {
() => {
// Module: crate::test_runner::result_cache
// Provides: {"ResultCache"}
// Dependencies: {}
# [doc = " An object which can cache the outcomes of tests."] pub trait ResultCache { # [doc = " Convert the given cache key into a `u64` representing that value. The"] # [doc = " u64 is used as the key below."] # [doc = ""] # [doc = " This is a separate step so that ownership of the key value can be"] # [doc = " handed off to user code without needing to be able to clone it."] fn key (& self , key : & ResultCacheKey) -> u64 ; # [doc = " Save `result` as the outcome associated with the test input in `key`."] # [doc = ""] # [doc = " `result` is passed as a reference so that the decision to clone depends"] # [doc = " on whether the cache actually plans on storing it."] fn put (& mut self , key : u64 , result : & TestCaseResult) ; # [doc = " If `put()` has been called with a semantically equivalent `key`, return"] # [doc = " the saved result. Otherwise, return `None`."] fn get (& self , key : u64) -> Option < & TestCaseResult > ; }
};
}
