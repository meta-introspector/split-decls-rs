// Generated macro for test_json_render (function)
macro_rules! Depcrate_json_valuetest_json_render {
() => {
// Module: crate::json::value
// Provides: {"test_json_render"}
// Dependencies: {}
# [test] fn test_json_render () { let raw = "<p>Hello world</p>\n<p thing=\"hello\"</p>" ; let thing = Json :: String (raw . to_string ()) ; assert_eq ! (raw , thing . render ()) ; }
};
}
