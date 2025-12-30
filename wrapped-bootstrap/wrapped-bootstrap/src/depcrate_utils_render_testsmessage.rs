// Generated macro for Message (enum)
macro_rules! Depcrate_utils_render_testsMessage {
() => {
// Module: crate::utils::render_tests
// Provides: {"Message"}
// Dependencies: {}
# [derive (serde_derive :: Deserialize)] # [serde (tag = "type" , rename_all = "snake_case")] enum Message { Suite (SuiteMessage) , Test (TestMessage) , Bench (BenchOutcome) , }
};
}
