// Generated macro for impl_1205 (impl)
macro_rules! Depcrate_test_runner_result_cacheimpl_1205 {
() => {
// Module: crate::test_runner::result_cache
// Provides: {"impl_1205"}
// Dependencies: {}
impl < 'a > ResultCacheKey < 'a > { pub (crate) fn new (value : & 'a dyn fmt :: Debug) -> Self { Self { value } } # [doc = " Return the test input value as an `&dyn Debug`."] pub fn value_debug (& self) -> & dyn fmt :: Debug { self . value } }
};
}
