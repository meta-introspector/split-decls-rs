// Generated macro for TestMessage (enum)
macro_rules! Depcrate_utils_render_testsTestMessage {
() => {
// Module: crate::utils::render_tests
// Provides: {"TestMessage"}
// Dependencies: {}
# [derive (serde_derive :: Deserialize)] # [serde (tag = "event" , rename_all = "snake_case")] enum TestMessage { Ok (TestOutcome) , Failed (TestOutcome) , Ignored (TestOutcome) , Timeout { name : String } , Started , }
};
}
