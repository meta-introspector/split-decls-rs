// Generated macro for Test (enum)
macro_rules! Depcrate_format_testTest {
() => {
// Module: crate::format::test
// Provides: {"Test"}
// Dependencies: {}
# [doc = " Test case event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [serde (rename_all = "snake_case")] # [serde (tag = "event")] pub enum Test { # [doc = " Case-started event."] Started (TestStarted) , # [doc = " Case-finished successfully event."] Ok (TestOk) , # [doc = " Case-finished with failure event."] Failed (TestFailed) , # [doc = " Case-ignored event."] Ignored (TestIgnored) , # [doc = " Case-allowed-failure event."] AllowedFailure (TestAllowedFailure) , # [doc = " Case-timeout event."] Timeout (TestTimeout) , # [cfg (not (feature = "strict_unstable"))] # [doc (hidden)] # [serde (other)] Unknown , }
};
}
