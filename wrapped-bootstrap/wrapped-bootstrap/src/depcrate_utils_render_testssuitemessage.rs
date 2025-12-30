// Generated macro for SuiteMessage (enum)
macro_rules! Depcrate_utils_render_testsSuiteMessage {
() => {
// Module: crate::utils::render_tests
// Provides: {"SuiteMessage"}
// Dependencies: {}
# [derive (serde_derive :: Deserialize)] # [serde (tag = "event" , rename_all = "snake_case")] enum SuiteMessage { Ok (SuiteOutcome) , Failed (SuiteOutcome) , Started { test_count : usize } , }
};
}
