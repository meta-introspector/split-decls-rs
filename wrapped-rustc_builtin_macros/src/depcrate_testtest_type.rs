// Generated macro for test_type (function)
macro_rules! Depcrate_testtest_type {
() => {
// Module: crate::test
// Provides: {"test_type"}
// Dependencies: {}
# [doc = " Attempts to determine the type of test."] # [doc = " Since doctests are created without macro expanding, only possible variants here"] # [doc = " are `UnitTest`, `IntegrationTest` or `Unknown`."] fn test_type (cx : & ExtCtxt < '_ >) -> TestType { let crate_path = cx . root_path . as_path () ; if crate_path . ends_with ("src") { TestType :: UnitTest } else if crate_path . ends_with ("tests") { TestType :: IntegrationTest } else { TestType :: Unknown } }
};
}
