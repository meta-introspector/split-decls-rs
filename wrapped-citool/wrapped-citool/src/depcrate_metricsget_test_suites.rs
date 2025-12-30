// Generated macro for get_test_suites (function)
macro_rules! Depcrate_metricsget_test_suites {
() => {
// Module: crate::metrics
// Provides: {"get_test_suites"}
// Dependencies: {}
pub fn get_test_suites (metrics : & JsonRoot) -> Vec < & TestSuite > { fn visit_test_suites < 'a > (nodes : & 'a [JsonNode] , suites : & mut Vec < & 'a TestSuite >) { for node in nodes { match node { JsonNode :: RustbuildStep { children , .. } => { visit_test_suites (& children , suites) ; } JsonNode :: TestSuite (suite) => { suites . push (& suite) ; } } } } let mut suites = vec ! [] ; for invocation in & metrics . invocations { visit_test_suites (& invocation . children , & mut suites) ; } suites }
};
}
