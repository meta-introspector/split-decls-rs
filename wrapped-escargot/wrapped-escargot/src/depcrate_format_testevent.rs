// Generated macro for Event (enum)
macro_rules! Depcrate_format_testEvent {
() => {
// Module: crate::format::test
// Provides: {"Event"}
// Dependencies: {}
# [doc = " Test-runner event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [serde (rename_all = "snake_case")] # [serde (tag = "type")] pub enum Event { # [doc = " Suite event."] Suite (Suite) , # [doc = " Test case event."] Test (Test) , # [doc = " Benchmark event."] Bench (Bench) , # [cfg (not (feature = "strict_unstable"))] # [doc (hidden)] # [serde (other)] Unknown , }
};
}
