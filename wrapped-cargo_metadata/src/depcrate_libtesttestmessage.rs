// Generated macro for TestMessage (enum)
macro_rules! Depcrate_libtestTestMessage {
() => {
// Module: crate::libtest
// Provides: {"TestMessage"}
// Dependencies: {}
# [derive (Debug , PartialEq , Deserialize , Serialize)] # [doc = " Represents the output of `cargo test -- -Zunstable-options --report-time --show-output --format json`."] # [doc = ""] # [doc = " requires --report-time"] # [doc = ""] # [doc = " # Stability"] # [doc = ""] # [doc = " As this struct is for interfacing with the unstable libtest json output, this struct may change at any time, without semver guarantees."] # [serde (tag = "type")] # [serde (rename_all = "lowercase")] pub enum TestMessage { # [doc = " suite related message"] Suite (SuiteEvent) , # [doc = " test related message"] Test (TestEvent) , # [doc = " bench related message"] Bench { # [doc = " name of benchmark"] name : String , # [doc = " distribution"] median : f32 , # [doc = " deviation"] deviation : f32 , # [doc = " thruput in MiB per second"] mib_per_second : Option < f32 > , } , }
};
}
