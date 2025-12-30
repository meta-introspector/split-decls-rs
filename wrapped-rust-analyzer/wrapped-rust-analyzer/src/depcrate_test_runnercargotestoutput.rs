// Generated macro for CargoTestOutput (enum)
macro_rules! Depcrate_test_runnerCargoTestOutput {
() => {
// Module: crate::test_runner
// Provides: {"CargoTestOutput"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (tag = "type" , rename_all = "camelCase")] pub (crate) enum CargoTestOutput { Test { name : String , # [serde (flatten)] state : TestState , } , Suite , Finished , Custom { text : String , } , }
};
}
