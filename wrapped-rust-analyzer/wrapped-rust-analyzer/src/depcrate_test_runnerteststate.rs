// Generated macro for TestState (enum)
macro_rules! Depcrate_test_runnerTestState {
() => {
// Module: crate::test_runner
// Provides: {"TestState"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (tag = "event" , rename_all = "camelCase")] pub (crate) enum TestState { Started , Ok , Ignored , Failed { # [serde (skip_serializing_if = "String::is_empty" , default)] stdout : String , } , }
};
}
