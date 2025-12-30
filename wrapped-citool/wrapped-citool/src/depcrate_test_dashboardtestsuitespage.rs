// Generated macro for TestSuitesPage (struct)
macro_rules! Depcrate_test_dashboardTestSuitesPage {
() => {
// Module: crate::test_dashboard
// Provides: {"TestSuitesPage"}
// Dependencies: {}
# [derive (Template)] # [template (path = "test_suites.askama")] struct TestSuitesPage < 'a > { suites : TestSuites < 'a > , test_count : u64 , }
};
}
