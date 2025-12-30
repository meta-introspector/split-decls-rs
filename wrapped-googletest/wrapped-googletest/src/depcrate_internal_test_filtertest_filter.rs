// Generated macro for TEST_FILTER (static)
macro_rules! Depcrate_internal_test_filterTEST_FILTER {
() => {
// Module: crate::internal::test_filter
// Provides: {"TEST_FILTER"}
// Dependencies: {}
static TEST_FILTER : OnceLock < Box < dyn TestFilter + Send + Sync > > = OnceLock :: new () ;
};
}
