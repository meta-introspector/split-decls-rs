// Generated macro for ResultCacheKey (struct)
macro_rules! Depcrate_test_runner_result_cacheResultCacheKey {
() => {
// Module: crate::test_runner::result_cache
// Provides: {"ResultCacheKey"}
// Dependencies: {}
# [doc = " A key used for the result cache."] # [doc = ""] # [doc = " The capabilities of this structure are currently quite limited; all one can"] # [doc = " do with safe code is get the `&dyn Debug` of the test input value. This may"] # [doc = " improve in the future, particularly at such a time that specialisation"] # [doc = " becomes stable."] # [derive (Debug)] pub struct ResultCacheKey < 'a > { value : & 'a dyn fmt :: Debug , }
};
}
